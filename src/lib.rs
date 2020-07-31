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
    fn get_keys(&self) -> Vec<u16>;
}

impl DeviceQuery for DeviceState {
    fn get_keys(&self) -> Vec<u16> {
        self.query_keymap()
    }
}
