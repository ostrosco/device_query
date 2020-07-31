extern crate user32;

use windows::user32::GetAsyncKeyState;

pub struct DeviceState;

impl DeviceState {
    pub fn new() -> DeviceState {
        DeviceState {}
    }

    pub fn query_keymap(&self) -> Vec<u16> {
        let mut keycodes = vec![];
        for key in 0_u16..256_u16 {
            unsafe {
                let state = GetAsyncKeyState(key as i32) as u32;
                if state & 0x8000 != 0 {
                    keycodes.push(key);
                }
            }
        }
        keycodes
    }
}
