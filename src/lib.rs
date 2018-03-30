//! A simple library for querying mouse and keyboard state without requiring
//! an active window. Currently works in Windows and Linux.

pub mod keymap;
pub mod mouse_state;
pub use keymap::Keycode;
pub use mouse_state::MouseState;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::DeviceState;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use windows::DeviceState;

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
