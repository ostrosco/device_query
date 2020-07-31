extern crate input_event_codes as kernel_key;
extern crate x11;

use linux::x11::xlib;
use std::ptr;
use std::slice;

pub struct DeviceState {
    display: *mut xlib::Display,
}

impl DeviceState {
    pub fn new() -> DeviceState {
        unsafe {
            let disp = xlib::XOpenDisplay(ptr::null());
            DeviceState { display: disp }
        }
    }

    pub fn query_keymap(&self) -> Vec<u16> {
        let mut keycodes = vec![];
        unsafe {
            let keymap: *mut i8 = [0; 32].as_mut_ptr();
            xlib::XQueryKeymap(self.display, keymap);
            for (ix, byte) in
                slice::from_raw_parts(keymap, 32).iter().enumerate()
            {
                for bit in 0_u8..8_u8 {
                    let bitmask = 1 << bit;
                    if byte & bitmask != 0 {
                        //x11 keycode uses kernel keycode with an offset of 8.
                        let x11_key = ix as u8 * 8 + bit;
                        let kernel_key = x11_key - 8;
                        keycodes.push(kernel_key as u16);
                    }
                }
            }
        }
        keycodes
    }
}
