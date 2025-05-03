//! Manages the application's input and window event loop lifecycle.
//!
//! `EventHandler` acts as the central hub for event dispatching, input interpretation,
//! and window management. It wraps the `winit` event loop, tracks registered windows,
//! and routes raw keyboard input into directional movement commands used by the game.
//!
//! ## Responsibilities
//! - Hosts and manages the main event loop via `winit`
//! - Tracks multiple windows by their `WindowId`
//! - Converts low-level input into high-level `Coordinate` events
//! - Notifies subscribers (e.g., gameplay logic) of movement input
//!
//! ## Design Principles
//! - Decouples platform event APIs from game logic using `GameInputHandler`
//! - Uses pub/sub pattern to notify listeners of input-driven movement
//! - Supports injection of custom `Window` implementations for flexibility
//!
//! ## Example Usage
//! ```no_run
//! let mut handler = EventHandler::new();
//! handler.register_window(Box::new(MyWindow::new(...)));
//! handler.start().unwrap(); // blocks forever
//! ```
use crossbeam::channel::{unbounded, Sender};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use winit::{
    error::EventLoopError,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowId,
};

use crate::input::{GameInputHandler, Input, PhysicalKeyInfo};
use crate::prelude::*;

/// Central manager for event dispatch and window tracking.
///
/// This struct owns the event loop and maintains a registry of windows.
/// It provides the glue between system-level events and game/application logic.
pub(crate) struct EventHandler {
    evtloop: EventLoop<()>,
    windows: HashMap<WindowId, Arc<Mutex<winit::window::Window>>>,
    input_handler: GameInputHandler,
    coordinate_subscribers: Vec<Sender<Coordinate>>,
}
impl EventHandler {
    /// Get the event handler with an empty window registry.
    ///
    /// This sets up the foundational state required to begin responding
    /// to system events, but does not yet start the event loop.
    /// Attempting to create the event loop off the main thread will panic.
    /// ## Panics
    ///
    /// Attempting to create the event loop off the main thread will panic. This
    /// restriction isn't strictly necessary on all platforms, but is imposed to
    /// eliminate any nasty surprises when porting to platforms that require it.
    /// `EventLoopBuilderExt::any_thread` functions are exposed in the relevant
    /// [`platform`] module if the target platform supports creating an event
    /// loop on any thread.
    ///
    /// Panics if created more than once
    pub(crate) fn new() -> EventHandler {
        Self {
            evtloop: EventLoop::new().unwrap(),
            windows: HashMap::default(),
            input_handler: GameInputHandler::default(),
            coordinate_subscribers: Vec::new(),
        }
    }
    /// Begins running the application's main event loop.
    ///
    /// This function blocks the current thread and drives all window
    /// and device events. Control is handed over to the system's event dispatcher.
    /// Intended to be called once after all setup is complete.
    pub(crate) fn start(mut self) -> Result<(), EventLoopError> {
        self.evtloop.run(move |event, target| {
            target.set_control_flow(ControlFlow::Wait);
            // dbg!(&event);
            match event {
                Event::WindowEvent { event, window_id } => match event {
                    // Listening for keyboard inputs
                    WindowEvent::KeyboardInput { event, .. } => {
                        let input = Input::PhysicalKey(PhysicalKeyInfo {
                            state: event.state,
                            code: event.physical_key,
                        });
                        let coordinate = self.input_handler.to_coordinate(input);
                        if let Some(c) = coordinate {
                            for sub in &self.coordinate_subscribers {
                                sub.try_send(c).unwrap()
                            }
                        }
                    }
                    // Exit Main Window
                    WindowEvent::CloseRequested => {
                        target.exit()
                    },
                    _ => (),
                },
                // Event::NewEvents(start_cause) => todo!(),
                // Event::DeviceEvent { device_id, event } => todo!(),
                // Event::UserEvent(_) => todo!(),
                // Event::Suspended => todo!(),
                // Event::Resumed => todo!(),
                // Event::AboutToWait => todo!(),
                // Event::LoopExiting => todo!(),
                // Event::MemoryWarning => todo!(),
                _ => (),
            }
        })
    }
    // Registers a new window to receive events.
    ///
    /// This allows the event loop to correctly dispatch input and OS events
    /// to the appropriate window handler based on the window's ID.
    pub(crate) fn register_window(&mut self, window: Arc<Mutex<winit::window::Window>>) {
        self.windows
            .insert(window.lock().unwrap().id(), window.clone());
    }
    /// Grants access to the underlying event loop instance.
    ///
    /// Useful when external components need to reference the event loop
    /// during the window-building phase.
    pub(crate) fn event_loop(&self) -> &EventLoop<()> {
        &self.evtloop
    }
    /// Registers a new subscriber to receive `Coordinate`.
    pub(crate) fn subscribe_coordinate(&mut self, subscriber: &mut dyn Subscriber<Coordinate>) {
        let (tx, rx) = unbounded::<Coordinate>();
        subscriber.subscribe(rx);
        self.coordinate_subscribers.push(tx);
    }
}
