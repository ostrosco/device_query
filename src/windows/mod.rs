extern crate user32;
extern crate winapi;

use windows::winapi::windef::POINT;
use windows::user32::GetCursorPos;

pub struct DeviceState;

impl DeviceState {
    pub fn new() -> DeviceState {
        DeviceState {}
    }

    pub fn query_pointer(&self) -> (i32, i32) {
        unsafe {
            let point = &mut POINT { x: 0, y: 0 };
            if GetCursorPos(point) != 0 {
                ((*point).x, (*point).y)
            } else {
                (0, 0)
            }
        }
    }
}
