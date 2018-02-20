#[cfg(target_os = "linux")]
pub mod linux;
pub use linux::MouseCoords;

pub trait MouseQuery {
    fn get_coords(&self) -> (i32, i32);
}

impl MouseQuery for MouseCoords {
    fn get_coords(&self) -> (i32, i32) {
        self.query_pointer()
    }
}
