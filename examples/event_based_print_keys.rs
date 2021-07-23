extern crate device_query;

use device_query::{DeviceEvents, DeviceState};

fn main() {
    let device_state = DeviceState::new();
    let _guard = device_state.on_key_down(|key| {
        println!("Down: {:#?}", key);
    });
    let _guard = device_state.on_key_up(|key| {
        println!("Up: {:#?}", key);
    });

    loop {}
}