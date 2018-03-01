extern crate mouse_coords;

use mouse_coords::{MouseCoords, MouseQuery};

fn main() {
    let coords = MouseCoords::new();
    loop {
        // println!("{:?}", coords.get_coords());
        coords.get_keys();
    }
}
