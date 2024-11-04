# device_query

[![Build Status](https://travis-ci.org/ostrosco/device_query.svg?branch=master)](https://travis-ci.org/ostrosco/device_query)

A simple library to query mouse and keyboard inputs on demand without a window.
Will work in Windows, Linux on X11, and macOS.

```Rust
use device_query::{DeviceQuery, DeviceState, MouseState, Keycode};

let device_state = DeviceState::new();
let mouse: MouseState = device_state.get_mouse();
println!("Current Mouse Coordinates: {:?}", mouse.coords);
let keys: Vec<Keycode> = device_state.get_keys();
println!("Is A pressed? {}", keys.contains(Keycode::A));
```

# Dependencies

Windows shouldn't require any special software to be installed for `device_query` to work properly.
On Linux, the X11 development libraries are required for `device_query` to query state from the OS.

On Ubuntu/Debian:
```
sudo apt install libx11-dev
```

On Fedora/RHEL/CentOS:
```
sudo dnf install xorg-x11-server-devel
```

On newer versions of MacOS, you may run into issues where you only see meta keys such as shift,
backspace, et cetera. This is due to a permission issue. To work around this:

* open the MacOS system preferences
* go to Security -> Privacy
* scroll down to Accessibility and unlock it
* add the app that is using `device_query` (such as your terminal) to the list

# Device Callbacks

`device_query` allows you to register callbacks for various device events such as key presses and mouse movements.

## Example

Here's a simple example demonstrating how to use the callback system:

```rust
extern crate device_query;
use device_query::{DeviceEvents, DeviceEventsHandler, Keycode, MouseButton, MousePosition};
use std::thread;
use std::time::Duration;

fn main() {
    // Initialize the event handler with a sleep duration of 10 milliseconds
    let event_handler = DeviceEventsHandler::new(Duration::from_millis(10))
        .expect("Could not initialize event loop");

    // Register callbacks for various events
    // The callbacks will be automatically deregistered when they go out of scope
    let _mouse_move_guard = event_handler.on_mouse_move(|position: &MousePosition| {
        println!("Mouse moved to position: {:?}", position);
    });

    // Keep the main thread alive to continue receiving events
    loop {
        thread::sleep(Duration::from_secs(1000));
    }
}
```