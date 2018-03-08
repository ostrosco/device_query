extern crate device_query;

use device_query::{DeviceState, DeviceQuery};

fn main() {
    let coords = DeviceState::new();
    loop {
        println!("{:?}", coords.get_coords());
        println!("{:?}", coords.get_keys());
    }
}
