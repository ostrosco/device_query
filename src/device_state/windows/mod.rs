use keymap::Keycode;
use mouse_state::MouseState;
use windows::Win32::Foundation::POINT;
use windows::Win32::UI::Input::KeyboardAndMouse;
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VIRTUAL_KEY};
use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;

#[derive(Debug, Clone)]
pub struct DeviceState;

impl DeviceState {
    pub fn new() -> Self {
        Self {}
    }

    // Adding because Linux and OSX supports this where `new` can panic.
    pub fn checked_new() -> Option<Self> {
        Some(Self::new())
    }

    pub fn query_pointer(&self) -> MouseState {
        let point = &mut POINT { x: 0, y: 0 };
        let button1pressed;
        let button2pressed;
        let button3pressed;
        let button4pressed;
        let button5pressed;
        let coords;
        unsafe {
            coords = if GetCursorPos(point).into() {
                (point.x, point.y)
            } else {
                (0, 0)
            };
            button1pressed =
                GetAsyncKeyState(KeyboardAndMouse::VK_LBUTTON.0 as i32) as u32 & 0x8000 != 0;
            button2pressed =
                GetAsyncKeyState(KeyboardAndMouse::VK_RBUTTON.0 as i32) as u32 & 0x8000 != 0;
            button3pressed =
                GetAsyncKeyState(KeyboardAndMouse::VK_MBUTTON.0 as i32) as u32 & 0x8000 != 0;
            button4pressed =
                GetAsyncKeyState(KeyboardAndMouse::VK_XBUTTON1.0 as i32) as u32 & 0x8000 != 0;
            button5pressed =
                GetAsyncKeyState(KeyboardAndMouse::VK_XBUTTON2.0 as i32) as u32 & 0x8000 != 0;
        }
        MouseState {
            coords,
            button_pressed: vec![
                false,
                button1pressed,
                button2pressed,
                button3pressed,
                button4pressed,
                button5pressed,
            ],
        }
    }

    pub fn query_keymap(&self) -> Vec<Keycode> {
        let mut keycodes = vec![];
        let mut keymap = vec![];
        unsafe {
            for key in 0..256 {
                keymap.push(GetAsyncKeyState(key));
            }
        }
        for (ix, byte) in keymap.iter().enumerate() {
            if *byte as u32 & 0x8000 != 0 {
                if let Some(k) = self.win_key_to_keycode(ix as u16) {
                    keycodes.push(k)
                }
            }
        }
        keycodes
    }

    fn win_key_to_keycode(&self, win_key: u16) -> Option<Keycode> {
        let mut keycode = match VIRTUAL_KEY(win_key) {
            KeyboardAndMouse::VK_F1 => Some(Keycode::F1),
            KeyboardAndMouse::VK_F2 => Some(Keycode::F2),
            KeyboardAndMouse::VK_F3 => Some(Keycode::F3),
            KeyboardAndMouse::VK_F4 => Some(Keycode::F4),
            KeyboardAndMouse::VK_F5 => Some(Keycode::F5),
            KeyboardAndMouse::VK_F6 => Some(Keycode::F6),
            KeyboardAndMouse::VK_F7 => Some(Keycode::F7),
            KeyboardAndMouse::VK_F8 => Some(Keycode::F8),
            KeyboardAndMouse::VK_F9 => Some(Keycode::F9),
            KeyboardAndMouse::VK_F10 => Some(Keycode::F10),
            KeyboardAndMouse::VK_F11 => Some(Keycode::F11),
            KeyboardAndMouse::VK_F12 => Some(Keycode::F12),
            KeyboardAndMouse::VK_NUMPAD0 => Some(Keycode::Numpad0),
            KeyboardAndMouse::VK_NUMPAD1 => Some(Keycode::Numpad1),
            KeyboardAndMouse::VK_NUMPAD2 => Some(Keycode::Numpad2),
            KeyboardAndMouse::VK_NUMPAD3 => Some(Keycode::Numpad3),
            KeyboardAndMouse::VK_NUMPAD4 => Some(Keycode::Numpad4),
            KeyboardAndMouse::VK_NUMPAD5 => Some(Keycode::Numpad5),
            KeyboardAndMouse::VK_NUMPAD6 => Some(Keycode::Numpad6),
            KeyboardAndMouse::VK_NUMPAD7 => Some(Keycode::Numpad7),
            KeyboardAndMouse::VK_NUMPAD8 => Some(Keycode::Numpad8),
            KeyboardAndMouse::VK_NUMPAD9 => Some(Keycode::Numpad9),
            KeyboardAndMouse::VK_ADD => Some(Keycode::NumpadAdd),
            KeyboardAndMouse::VK_SUBTRACT => Some(Keycode::NumpadSubtract),
            KeyboardAndMouse::VK_DIVIDE => Some(Keycode::NumpadDivide),
            KeyboardAndMouse::VK_MULTIPLY => Some(Keycode::NumpadMultiply),
            KeyboardAndMouse::VK_OEM_NEC_EQUAL => Some(Keycode::NumpadEquals),
            KeyboardAndMouse::VK_DECIMAL => Some(Keycode::NumpadDecimal),
            KeyboardAndMouse::VK_SPACE => Some(Keycode::Space),
            KeyboardAndMouse::VK_LCONTROL => Some(Keycode::LControl),
            KeyboardAndMouse::VK_RCONTROL => Some(Keycode::RControl),
            KeyboardAndMouse::VK_LSHIFT => Some(Keycode::LShift),
            KeyboardAndMouse::VK_RSHIFT => Some(Keycode::RShift),
            KeyboardAndMouse::VK_LMENU => Some(Keycode::LAlt),
            KeyboardAndMouse::VK_RMENU => Some(Keycode::RAlt),
            KeyboardAndMouse::VK_LWIN => Some(Keycode::LMeta),
            KeyboardAndMouse::VK_RWIN => Some(Keycode::RMeta),
            KeyboardAndMouse::VK_RETURN => Some(Keycode::Enter),
            KeyboardAndMouse::VK_ESCAPE => Some(Keycode::Escape),
            KeyboardAndMouse::VK_UP => Some(Keycode::Up),
            KeyboardAndMouse::VK_DOWN => Some(Keycode::Down),
            KeyboardAndMouse::VK_LEFT => Some(Keycode::Left),
            KeyboardAndMouse::VK_RIGHT => Some(Keycode::Right),
            KeyboardAndMouse::VK_BACK => Some(Keycode::Backspace),
            KeyboardAndMouse::VK_CAPITAL => Some(Keycode::CapsLock),
            KeyboardAndMouse::VK_TAB => Some(Keycode::Tab),
            KeyboardAndMouse::VK_HOME => Some(Keycode::Home),
            KeyboardAndMouse::VK_END => Some(Keycode::End),
            KeyboardAndMouse::VK_PRIOR => Some(Keycode::PageUp),
            KeyboardAndMouse::VK_NEXT => Some(Keycode::PageDown),
            KeyboardAndMouse::VK_INSERT => Some(Keycode::Insert),
            KeyboardAndMouse::VK_DELETE => Some(Keycode::Delete),
            KeyboardAndMouse::VK_OEM_3 => Some(Keycode::Grave),
            KeyboardAndMouse::VK_OEM_MINUS => Some(Keycode::Minus),
            KeyboardAndMouse::VK_OEM_PLUS => Some(Keycode::Equal),
            KeyboardAndMouse::VK_OEM_4 => Some(Keycode::LeftBracket),
            KeyboardAndMouse::VK_OEM_6 => Some(Keycode::RightBracket),
            KeyboardAndMouse::VK_OEM_5 => Some(Keycode::BackSlash),
            KeyboardAndMouse::VK_OEM_1 => Some(Keycode::Semicolon),
            KeyboardAndMouse::VK_OEM_7 => Some(Keycode::Apostrophe),
            KeyboardAndMouse::VK_OEM_COMMA => Some(Keycode::Comma),
            KeyboardAndMouse::VK_OEM_PERIOD => Some(Keycode::Dot),
            KeyboardAndMouse::VK_OEM_2 => Some(Keycode::Slash),

            _ => None,
        };

        if keycode.is_none() {
            let win_key = win_key as u8;
            keycode = match win_key as char {
                '0' => Some(Keycode::Key0),
                '1' => Some(Keycode::Key1),
                '2' => Some(Keycode::Key2),
                '3' => Some(Keycode::Key3),
                '4' => Some(Keycode::Key4),
                '5' => Some(Keycode::Key5),
                '6' => Some(Keycode::Key6),
                '7' => Some(Keycode::Key7),
                '8' => Some(Keycode::Key8),
                '9' => Some(Keycode::Key9),
                'A' => Some(Keycode::A),
                'B' => Some(Keycode::B),
                'C' => Some(Keycode::C),
                'D' => Some(Keycode::D),
                'E' => Some(Keycode::E),
                'F' => Some(Keycode::F),
                'G' => Some(Keycode::G),
                'H' => Some(Keycode::H),
                'I' => Some(Keycode::I),
                'J' => Some(Keycode::J),
                'K' => Some(Keycode::K),
                'L' => Some(Keycode::L),
                'M' => Some(Keycode::M),
                'N' => Some(Keycode::N),
                'O' => Some(Keycode::O),
                'P' => Some(Keycode::P),
                'Q' => Some(Keycode::Q),
                'R' => Some(Keycode::R),
                'S' => Some(Keycode::S),
                'T' => Some(Keycode::T),
                'U' => Some(Keycode::U),
                'V' => Some(Keycode::V),
                'W' => Some(Keycode::W),
                'X' => Some(Keycode::X),
                'Y' => Some(Keycode::Y),
                'Z' => Some(Keycode::Z),
                _ => None,
            }
        }
        keycode
    }
}
