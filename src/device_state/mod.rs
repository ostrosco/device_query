//! DeviceState implementation.
//! 
//! This module contains the implementation of the DeviceState struct.
//! This only allows to get the current state of the device.
//! for callbacks, see the [`DeviceEventsHandler`](crate::device_events::DeviceEventsHandler) struct.
//! 
//! # Example
//! 
//! ```no_run
//! use device_query::{DeviceState, DeviceQuery};
//! 
//! fn main() {
//!   let device_state = DeviceState::new(); 
//!   println!("Mouse position: {:?}", device_state.get_mouse());
//!   println!("Key down: {:?}", device_state.get_keys());
//! }
//! 
//! ```

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use self::linux::DeviceState;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use self::windows::DeviceState;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use self::macos::DeviceState;

impl Default for DeviceState {
    fn default() -> Self {
        Self::new()
    }
}
