use macroquad::prelude::*;

use crate::grid::{Grid, WIDTH, HEIGHT};

pub fn draw_grid(grid: &Grid, pixel_size: f32) {
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let pixel_x = x as f32 * pixel_size;
            let pixel_y = y as f32 * pixel_size;

            let value = grid.get_cell(x, y);
            let colour = match value {
                0 => BLACK,
                1 => WHITE,
                _ => BLUE,
            };

            draw_rectangle(pixel_x, pixel_y, pixel_size, pixel_size, colour);
        }
    }
}
