extern crate user32;
extern crate winapi;

use keymap::Keycode;
use windows::winapi::windef::POINT;
use windows::winapi::winuser;
use windows::user32::{GetAsyncKeyState, GetCursorPos};
use mouse_state::MouseState;

pub struct DeviceState;

impl DeviceState {
    pub fn new() -> DeviceState {
        DeviceState {}
    }

    pub fn query_pointer(&self) -> MouseState {
        unsafe {
            let point = &mut POINT { x: 0, y: 0 };
            let coords = if GetCursorPos(point) != 0 {
                ((*point).x, (*point).y)
            } else {
                (0, 0)
            };
            let button1pressed =
                GetAsyncKeyState(winuser::VK_LBUTTON) as u32 & 0x8000 != 0;
            let button2pressed =
                GetAsyncKeyState(winuser::VK_RBUTTON) as u32 & 0x8000 != 0;
            let button3pressed =
                GetAsyncKeyState(winuser::VK_MBUTTON) as u32 & 0x8000 != 0;
            let button4pressed =
                GetAsyncKeyState(winuser::VK_XBUTTON1) as u32 & 0x8000 != 0;
            let button5pressed =
                GetAsyncKeyState(winuser::VK_XBUTTON2) as u32 & 0x8000 != 0;
            MouseState {
                coords: coords,
                button_pressed: vec![
                    false,
                    button1pressed,
                    button2pressed,
                    button3pressed,
                    button4pressed,
                    button5pressed,
                ],
            }
        }
    }

    pub fn query_keymap(&self) -> Vec<Keycode> {
        unsafe {
            let mut keycodes = vec![];
            let mut keymap = vec![];
            for key in 0..256 {
                keymap.push(GetAsyncKeyState(key));
            }
            for (ix, byte) in keymap.iter().enumerate() {
                if *byte as u32 & 0x8000 != 0 {
                    match self.keycode_to_key(ix as i32) {
                        Some(k) => keycodes.push(k),
                        None => (),
                    }
                }
            }
            keycodes
        }
    }

    fn keycode_to_key(&self, keycode: i32) -> Option<Keycode> {
        match keycode {
            winuser::VK_SPACE => Some(Keycode::Space),
            winuser::VK_RETURN => Some(Keycode::Enter),
            _ => None,
        }
    }
}
