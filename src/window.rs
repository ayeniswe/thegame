//! A module for creating and managing a window with pixel-based rendering.
//!
//! This module provides a `GameWindow` struct that wraps around a `winit` window and integrates
//! it with the `pixels` crate for retro-style, pixel-perfect rendering. The `Window` trait defines
//! a common interface for accessing and interacting with the window, and the `Screen` trait allows
//! for rendering graphics and manipulating pixel data on the window surface.
//!
//! Key Types:
//! - `GameWindow`: A window that integrates pixel rendering, supporting fixed sizes and pixel-perfect
//! rendering, ideal for games or applications with low-resolution graphics.
//! - `Window`: A trait that abstracts common window operations, allowing different window types to
//! conform to a unified API for interaction.
//! - `Screen`: A trait that allows manipulation of the screen's framebuffer, enabling pixel drawing
//! and access to the window's dimensions.
//!
//! Errors:
//! - `WindowError`: A custom error type that captures potential errors that can occur during window
//! creation or pixel surface setup.

use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use std::sync::{Arc, Mutex};
use thiserror::Error;
use winit::{
    dpi::LogicalSize,
    window::{WindowBuilder, WindowId},
};

use crate::EventHandler;

/// Represents a generic abstraction over a window.
///
/// The `Window` trait allows different kinds of window implementations
/// to expose a common interface for identification and interaction.
pub trait Window {
    fn id(&self) -> WindowId;
}

/// A concrete implementation of the `Screen` trait backed by a pixel buffer.
///
/// `GameWindowScreen` is responsible for drawing to a pixel-based surface,
/// such as a game window or off-screen framebuffer. It maintains the dimensions
/// of the render area and the actual `Pixels` surface used for rendering.
///
pub struct GameWindowScreen {
    width: u32,
    height: u32,
    surface: Pixels,
}
impl Screen for GameWindowScreen {
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn frame_buffer(&mut self) -> &mut [u8] {
        self.surface.frame_mut()
    }
    fn clear(&mut self) -> Result<(), WindowError> {
        let frame = self.surface.frame_mut();
        // Clear the frame by setting all pixels to black (may flicker)
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[0, 0, 0, 255]); // RGBA black
        }
        Ok(())
    }
    fn render(&mut self) -> Result<(), WindowError> {
        self.surface.render()?;
        Ok(())
    }
}

/// Configures and builds a fixed-size window for a game with pixel rendering.
///
/// The `GameWindow` is for creating a window that's suitable
/// for retro-style or low-resolution games, where fixed dimensions and pixel-perfect
/// rendering are important.
pub(crate) struct GameWindow<'a> {
    inner: Arc<Mutex<winit::window::Window>>,
    screen: Arc<Mutex<GameWindowScreen>>,
    title: String,
    evt: &'a EventHandler,
}
impl<'a> GameWindow<'a> {
    /// Creates a new configuration for a `GameWindow`.
    ///
    /// Constructs the actual OS window and sets up the pixel rendering surface.
    ///
    /// Scaling happens by a `2.0` factor
    pub(crate) fn new(
        width: u32,
        height: u32,
        title: String,
        evt: &'a EventHandler,
    ) -> Result<Self, WindowError> {
        let pixel_size = LogicalSize::new(width, height);
        let window_size = pixel_size.to_physical(4.0);
        // Base cross-platform windowing for game view
        let window = WindowBuilder::new()
            .with_title(title.clone())
            .with_inner_size(window_size)
            .with_resizable(false)
            .with_min_inner_size(pixel_size)
            .build(evt.event_loop())?;

        // Logical texture to render pixels
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let surface =
            PixelsBuilder::new(pixel_size.width, pixel_size.height, surface_texture).build()?;

        Ok(Self {
            screen: Arc::new(Mutex::new(GameWindowScreen {
                width: pixel_size.width,
                height: pixel_size.height,
                surface,
            })),
            inner: Arc::new(Mutex::new(window)),
            title: title.into(),
            evt,
        })
    }
    pub(crate) fn screen(&self) -> Arc<Mutex<GameWindowScreen>> {
        self.screen.clone()
    }
    pub(crate) fn window(&mut self) -> Arc<Mutex<winit::window::Window>> {
        self.inner.clone()
    }
}
impl Window for winit::window::Window {
    fn id(&self) -> WindowId {
        self.id()
    }
}

/// The `Screen` trait defines the essential methods required for interacting with a screen or framebuffer.
/// Implementing this trait allows a type to expose properties which can be used for rendering graphics or manipulating pixel data.
pub trait Screen: Send + 'static {
    fn clear(&mut self) -> Result<(), WindowError>;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn frame_buffer(&mut self) -> &mut [u8];
    fn render(&mut self) -> Result<(), WindowError>;
}

#[derive(Debug, Error)]
pub enum WindowError {
    #[error("window creation failed: {0}")]
    WindowCreationError(#[from] winit::error::OsError),
    #[error("pixels surface texture setup failed: {0}")]
    PixelsCreationError(#[from] pixels::Error),
    #[error("failed to lock screen: {0}")]
    ScreenLockError(String),
}
