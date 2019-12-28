//! A simple library for querying mouse and keyboard state without requiring
//! an active window. Currently works in Windows, Linux, and macOS.
//!
//! ```no-run
//! use device_query::{DeviceQuery, DeviceState, MouseState, Keycode};
//!
//! let device_state = DeviceState::new();
//! let mouse: MouseState = device_state.get_mouse();
//! println!("Current Mouse Coordinates: {:?}", mouse.coords);
//! let keys: Vec<Keycode> = device_state.get_keys();
//! println!("Is A pressed? {}", keys.contains(&Keycode::A));
//! ```

pub mod keymap;
pub mod mouse_state;
pub use keymap::Keycode;
pub use mouse_state::MouseState;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::DeviceState;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::DeviceState;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::DeviceState;

pub trait DeviceQuery {
    fn get_mouse(&self) -> MouseState;
    fn get_keys(&self) -> Vec<Keycode>;
}

impl DeviceQuery for DeviceState {
    /// Query for the current mouse position and mouse button state.
    fn get_mouse(&self) -> MouseState {
        self.query_pointer()
    }

    /// Query for all keys that are currently pressed down.
    fn get_keys(&self) -> Vec<Keycode> {
        self.query_keymap()
    }
}
