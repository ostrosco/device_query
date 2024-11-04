extern crate device_query;

use device_query::{DeviceEvents, DeviceEventsHandler};
use std::thread;
use std::time::Duration;

fn main() {
    let event_handler = DeviceEventsHandler::new(std::time::Duration::from_millis(10))
        .expect("Could not initialize event loop");
    let _guard = event_handler.on_mouse_move(|position| {
        println!("Position: {:#?}", position);
    });
    let _guard = event_handler.on_mouse_down(|button| {
        println!("Down: {:#?}", button);
    });
    let _guard = event_handler.on_mouse_up(|button| {
        println!("Up: {:#?}", button);
    });

    loop {
        thread::sleep(Duration::from_secs(1000));
    }
}
