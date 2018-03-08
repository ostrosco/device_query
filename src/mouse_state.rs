#[derive(Debug)]
pub struct MouseState {
    pub coords: (i32, i32),
    pub button_pressed: Vec<bool>,
}
