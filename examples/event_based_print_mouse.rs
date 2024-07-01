extern crate device_query;

use device_query::{DeviceEvents, DeviceState};
use std::thread;
use std::time::Duration;

fn main() {
    if let Some(device_state) = DeviceState::new() {
        let _guard = device_state.on_mouse_move(|position| {
            println!("Position: {:#?}", position);
        });
        let _guard = device_state.on_mouse_down(|button| {
            println!("Down: {:#?}", button);
        });
        let _guard = device_state.on_mouse_up(|button| {
            println!("Up: {:#?}", button);
        });

        loop {
            thread::sleep(Duration::from_secs(1000));
        }
    }

    panic!("Could not create DeviceState")
}
