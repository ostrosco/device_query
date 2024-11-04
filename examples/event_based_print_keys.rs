extern crate device_query;

use device_query::{DeviceEvents, DeviceEventsHandler, DeviceState};
use std::thread;
use std::time::Duration;

fn main() {
    let event_handler = DeviceEventsHandler::new(Duration::from_millis(10))
        .expect("Could not initialize event loop");
    let _guard = event_handler.on_key_down(|key| {
        println!("Down: {:#?}", key);
    });
    let _guard = event_handler.on_key_up(|key| {
        println!("Up: {:#?}", key);
    });

    loop {
        thread::sleep(Duration::from_secs(1000));
    }
}
