extern crate device_query;

use device_query::{DeviceEvents, DeviceState};
use std::thread;
use std::time::Duration;

fn main() {
    if let Some(device_state) = DeviceState::new() {
        let _guard = device_state.on_key_down(|key| {
            println!("Down: {:#?}", key);
        });
        let _guard = device_state.on_key_up(|key| {
            println!("Up: {:#?}", key);
        });

        loop {
            thread::sleep(Duration::from_secs(1000));
        }
    }

    panic!("Could not create DeviceState")
}
