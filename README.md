# keyboard_query

A simple library to query keyboard inputs on demand without a window.
Will work in Windows, Linux on X11, and macOS.

## Example

```Rust
extern crate keyboard_query;

use keyboard_query::{DeviceQuery, DeviceState};

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
```

## Dependencies

On Ubuntu/Debian:

```bash
sudo apt install libx11-dev
```

On Fedora/RHEL/CentOS:

```bash
sudo dnf install xorg-x11-server-devel
```
