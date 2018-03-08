pub mod keymap;
pub mod mouse_state;
use keymap::Keycode;
use mouse_state::MouseState;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::DeviceState;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use windows::DeviceState;

pub trait DeviceQuery {
    fn get_coords(&self) -> MouseState;
    fn get_keys(&self) -> Vec<Keycode>;
}

impl DeviceQuery for DeviceState {
    fn get_coords(&self) -> MouseState {
        self.query_pointer()
    }

    fn get_keys(&self) -> Vec<Keycode> {
        self.query_keymap()
    }
}
