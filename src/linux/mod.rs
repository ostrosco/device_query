extern crate x11;

use linux::x11::xlib;
use std::ptr;
use keymap::Keycode;
use std::slice;

pub struct MouseCoords {
    display: *mut xlib::Display,
}

impl MouseCoords {
    pub fn new() -> MouseCoords {
        unsafe {
            xlib::XInitThreads();
            let disp = xlib::XOpenDisplay(ptr::null());
            MouseCoords {
                display: disp,
            }
        }
    }

    pub fn query_pointer(&self) -> (i32, i32) {
        unsafe {
            let root;
            root = xlib::XDefaultRootWindow(self.display);
            let mut root_x = 0;
            let mut root_y = 0;
            let mut win_x = 0;
            let mut win_y = 0;
            let mut root_return = 0;
            let mut child_return = 0;
            let mut mask_return = 0;
            xlib::XQueryPointer( self.display, root, &mut root_return, &mut child_return, 
                                         &mut root_x, &mut root_y,
                                         &mut win_x, &mut win_y, &mut mask_return);
            (win_x, win_y)
        }
    }

    pub fn query_keymap(&self) -> Vec<Keycode> {
        unsafe {
            let keymap: *mut i8 = [0; 32].as_mut_ptr();
            xlib::XQueryKeymap(self.display, keymap);
            for ix in slice::from_raw_parts(keymap, 32).iter() {
                print!("{}", ix);
            }
            println!("");
            vec![]
        }
    }
}
