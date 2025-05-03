//! A module providing synchronization utilities for event-driven systems.
//!
//! This module contains traits and structures designed for managing event-driven
//! communication between different parts of the system using channels. The primary
//! trait, `Subscriber`, allows for the subscription to a `Receiver<T>`, enabling
//! the handling of asynchronous events or messages of type `T` in a decoupled manner.
//!
//! # Key Features
//! - **Subscriber Trait**: Allows types to subscribe to a `Receiver<T>` and handle messages.
//! - **Crossbeam Channel**: Leverages `crossbeam::channel::Receiver` for efficient message passing.
//!
//! # Example Usage
//! A typical implementation of the `Subscriber` trait would look like this:
//! ```rust
//! struct MySubscriber;
//!
//! impl Subscriber<String> for MySubscriber {
//!     fn subscribe(&mut self, rx: Receiver<String>) {
//!         // Logic for handling received messages
//!     }
//! }
//! ```
use crossbeam::channel::Receiver;

/// A generic event subscriber that listens for incoming messages of type `T`
pub trait Subscriber<T> {
    fn subscribe(&mut self, rx: Receiver<T>);
}
