extern crate device_query;

use device_query::{DeviceQuery, DeviceState, MouseState};

fn main() {
    let device_state = DeviceState::new();
    let mut prev_coords = MouseState {coords: (0, 0), button_pressed: vec![]};
    loop {
        let coords = device_state.get_coords();
        if coords != prev_coords {
            println!("{:?}", coords);
        }
        prev_coords = coords;
    }
}
