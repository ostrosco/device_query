pub mod keymap;
pub mod mouse_state;
use keymap::Keycode;
use mouse_state::MouseState;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::MouseCoords;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use windows::MouseCoords;

pub trait MouseQuery {
    fn get_coords(&self) -> MouseState;
    fn get_keys(&self) -> Vec<Keycode>;
}

impl MouseQuery for MouseCoords {
    fn get_coords(&self) -> MouseState {
        self.query_pointer()
    }

    fn get_keys(&self) -> Vec<Keycode> {
        self.query_keymap()
    }
}
