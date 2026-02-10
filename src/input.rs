use macroquad::prelude::*;

use crate::grid::{Grid, WIDTH, HEIGHT};

pub fn paint_with_mouse(grid: &mut Grid, pixel_size: f32) {
    if !is_mouse_button_down(MouseButton::Left) {
        return;
    }

    let (mouse_x, mouse_y) = mouse_position();
    let x = (mouse_x / pixel_size).floor() as i32;
    let y = (mouse_y / pixel_size).floor() as i32;

}