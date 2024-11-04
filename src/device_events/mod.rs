//! Devices events listeners.
//! 
//! This module contains the implementation of the DeviceEventsHandler struct.
//! This allows to register callbacks for device events.
//! for the current state of the device, see the [`DeviceState`](crate::device_state::DeviceState) struct.
//! 
//! # Example
//! 
//! ```no_run
//! use device_query::{DeviceEvents, DeviceEventsHandler, Keycode, MouseButton};
//! use std::time::Duration;
//! 
//! fn main() {
//!   let device_events = DeviceEventsHandler::new(Duration::from_millis(10)).unwrap();
//!   // Register a key down event callback
//!   // The guard is used to keep the callback alive
//!   let _guard = device_events.on_key_down(|key| {
//!     println!("Key down: {:?}", key);
//!   });
//!   // Keep the main thread alive
//!   loop {}
//! }
//! 
//! ```
//! 

mod callback;
mod event_loop;
mod utils;

use std::time::Duration;

use crate::MousePosition;

pub use self::callback::*;
use self::event_loop::*;

use Keycode;
use MouseButton;

/// All the supported devices events.
pub trait DeviceEvents {
    /// Register an on key down event callback.
    fn on_key_down<Callback: Fn(&Keycode) + Sync + Send + 'static>(
        &self,
        callback: Callback,
    ) -> CallbackGuard<Callback>;
    /// Register an on key up event callback.
    fn on_key_up<Callback: Fn(&Keycode) + Sync + Send + 'static>(
        &self,
        callback: Callback,
    ) -> CallbackGuard<Callback>;

    /// Register an on mouse move event callback.
    fn on_mouse_move<Callback: Fn(&MousePosition) + Sync + Send + 'static>(
        &self,
        callback: Callback,
    ) -> CallbackGuard<Callback>;
    /// Register an on mouse button down event callback.
    fn on_mouse_down<Callback: Fn(&MouseButton) + Sync + Send + 'static>(
        &self,
        callback: Callback,
    ) -> CallbackGuard<Callback>;
    /// Register an on mouse button up event callback.
    fn on_mouse_up<Callback: Fn(&MouseButton) + Sync + Send + 'static>(
        &self,
        callback: Callback,
    ) -> CallbackGuard<Callback>;
}

pub struct DeviceEventsHandler;

impl DeviceEventsHandler {
    /// Attempts to start event loop with the given sleep duration.
    /// Returns None if the event loop is already running.
    pub fn new(sleep_dur: Duration) -> Option<Self> {
        event_loop::init_event_loop(sleep_dur).then_some(DeviceEventsHandler)
    }
}

/// Returns the event loop.
///
/// This is a workaround to avoid using unsafe code,
/// the existence of a [`DeviceEventsHandler`] means that the event loop is already initialized.
macro_rules! get_event_loop {
    () => {
        EVENT_LOOP
            .lock()
            .expect("Couldn't lock EVENT_LOOP")
            .as_mut()
            .unwrap()
    };
}

impl DeviceEvents for DeviceEventsHandler {
    fn on_key_down<Callback: Fn(&Keycode) + Sync + Send + 'static>(
        &self,
        callback: Callback,
    ) -> CallbackGuard<Callback> {
        get_event_loop!().on_key_down(callback)
    }

    fn on_key_up<Callback: Fn(&Keycode) + Sync + Send + 'static>(
        &self,
        callback: Callback,
    ) -> CallbackGuard<Callback> {
        get_event_loop!().on_key_up(callback)
    }

    fn on_mouse_move<Callback: Fn(&MousePosition) + Sync + Send + 'static>(
        &self,
        callback: Callback,
    ) -> CallbackGuard<Callback> {
        get_event_loop!().on_mouse_move(callback)
    }

    fn on_mouse_down<Callback: Fn(&MouseButton) + Sync + Send + 'static>(
        &self,
        callback: Callback,
    ) -> CallbackGuard<Callback> {
        get_event_loop!().on_mouse_down(callback)
    }

    fn on_mouse_up<Callback: Fn(&MouseButton) + Sync + Send + 'static>(
        &self,
        callback: Callback,
    ) -> CallbackGuard<Callback> {
        get_event_loop!().on_mouse_up(callback)
    }
}
