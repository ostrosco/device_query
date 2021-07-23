extern crate device_query;

use device_query::{DeviceEvents, DeviceState};

fn main() {
    let device_state = DeviceState::new();
    let _guard = device_state.on_mouse_move(|position| {
        println!("Position: {:#?}", position);
    });
    let _guard = device_state.on_mouse_down(|button| {
        println!("Down: {:#?}", button);
    });
    let _guard = device_state.on_mouse_up(|button| {
        println!("Up: {:#?}", button);
    });

    loop {}
}