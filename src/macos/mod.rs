use keymap::Keycode;
use mouse_state::MouseState;

pub struct DeviceState;

const MAPPING: &[(readkey::Keycode, Keycode)] = &[
    (readkey::Keycode::_0, Keycode::Key0),
    (readkey::Keycode::_1, Keycode::Key1),
    (readkey::Keycode::_2, Keycode::Key2),
    (readkey::Keycode::_3, Keycode::Key3),
    (readkey::Keycode::_4, Keycode::Key4),
    (readkey::Keycode::_5, Keycode::Key5),
    (readkey::Keycode::_6, Keycode::Key6),
    (readkey::Keycode::_7, Keycode::Key7),
    (readkey::Keycode::_8, Keycode::Key8),
    (readkey::Keycode::_9, Keycode::Key9),
    (readkey::Keycode::A, Keycode::A),
    (readkey::Keycode::B, Keycode::B),
    (readkey::Keycode::C, Keycode::C),
    (readkey::Keycode::D, Keycode::D),
    (readkey::Keycode::E, Keycode::E),
    (readkey::Keycode::F, Keycode::F),
    (readkey::Keycode::G, Keycode::G),
    (readkey::Keycode::H, Keycode::H),
    (readkey::Keycode::I, Keycode::I),
    (readkey::Keycode::J, Keycode::J),
    (readkey::Keycode::K, Keycode::K),
    (readkey::Keycode::L, Keycode::L),
    (readkey::Keycode::M, Keycode::M),
    (readkey::Keycode::N, Keycode::N),
    (readkey::Keycode::O, Keycode::O),
    (readkey::Keycode::P, Keycode::P),
    (readkey::Keycode::Q, Keycode::Q),
    (readkey::Keycode::R, Keycode::R),
    (readkey::Keycode::S, Keycode::S),
    (readkey::Keycode::T, Keycode::T),
    (readkey::Keycode::U, Keycode::U),
    (readkey::Keycode::V, Keycode::V),
    (readkey::Keycode::W, Keycode::W),
    (readkey::Keycode::X, Keycode::X),
    (readkey::Keycode::Y, Keycode::Y),
    (readkey::Keycode::Z, Keycode::Z),
    (readkey::Keycode::F1, Keycode::F1),
    (readkey::Keycode::F2, Keycode::F2),
    (readkey::Keycode::F3, Keycode::F3),
    (readkey::Keycode::F4, Keycode::F4),
    (readkey::Keycode::F5, Keycode::F5),
    (readkey::Keycode::F6, Keycode::F6),
    (readkey::Keycode::F7, Keycode::F7),
    (readkey::Keycode::F8, Keycode::F8),
    (readkey::Keycode::F9, Keycode::F9),
    (readkey::Keycode::F10, Keycode::F10),
    (readkey::Keycode::F11, Keycode::F11),
    (readkey::Keycode::F12, Keycode::F12),
    (readkey::Keycode::Escape, Keycode::Escape),
    (readkey::Keycode::Space, Keycode::Space),
    (readkey::Keycode::Control, Keycode::LControl),
    (readkey::Keycode::RightControl, Keycode::RControl),
    (readkey::Keycode::Shift, Keycode::LShift),
    (readkey::Keycode::RightShift, Keycode::RShift),
    (readkey::Keycode::Option, Keycode::LAlt),
    (readkey::Keycode::RightOption, Keycode::RAlt),
    (readkey::Keycode::Return, Keycode::Enter),
];

impl DeviceState {
    pub fn new() -> DeviceState {
        DeviceState {}
    }

    pub fn query_pointer(&self) -> MouseState {
        let (x, y) = readmouse::Mouse::location();
        let button_pressed = vec![
            false,
            readmouse::Mouse::Left.is_pressed(),
            readmouse::Mouse::Right.is_pressed(),
            readmouse::Mouse::Center.is_pressed(),
            false,
        ];
        MouseState {
            coords: (x as i32, y as i32),
            button_pressed,
        }
    }
    pub fn query_keymap(&self) -> Vec<Keycode> {
        MAPPING
            .iter()
            .filter(|(from, _)| from.is_pressed())
            .map(|(_, to)| to.clone())
            .collect()
    }
}
