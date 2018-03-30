extern crate device_query;

use device_query::{DeviceQuery, DeviceState};

fn main() {
    let device_state = DeviceState::new();
    let mut prev_keys = vec![];
    loop {
        let keys = device_state.get_keys();
        if keys != prev_keys {
            println!("{:?}", keys);
        }
        prev_keys = keys;
    }
}
