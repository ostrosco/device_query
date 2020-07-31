pub struct DeviceState;

#[link(name = "AppKit", kind = "framework")]
extern "C" {
    fn CGEventSourceKeyState(state: i32, keycode: u16) -> bool;
}

impl DeviceState {
    pub fn new() -> DeviceState {
        DeviceState {}
    }

    pub fn query_keymap(&self) -> Vec<u16> {
        let mut keycodes = vec![];
        for key in 0_u16..256_u16 {
            unsafe {
                if CGEventSourceKeyState(0, key) {
                    keycodes.push(key);
                }
            }
        }
        keycodes
    }
}
