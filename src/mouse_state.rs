//! Description of mouse coordinates and state of buttons.

/// Mouse position.
pub type MousePosition = (i32, i32);

/// MouseButton.
pub type MouseButton = usize;

#[derive(Debug, PartialEq, Default)]
/// A simple structure containing the current mouse coordinates and the
/// state of each mouse button that we can query. Currently, Windows and
/// Linux provide nice ways to query five mouse buttons. Since button
/// numbers are 1-based, `button_pressed[0]` is assumed to be false and
/// have no meaning.
pub struct MouseState {
    /// Coordinates in pixel.
    pub coords: MousePosition,
    /// State of each mouse button.
    pub button_pressed: Vec<bool>,
}
