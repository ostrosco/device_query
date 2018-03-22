extern crate device_query;

use device_query::{DeviceQuery, DeviceState};

fn main() {
    let coords = DeviceState::new();
    loop {
        println!("{:?}", coords.get_coords());
        println!("{:?}", coords.get_keys());
    }
}
