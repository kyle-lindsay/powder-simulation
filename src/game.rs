use macroquad::prelude::*;

use crate::{grid::Grid, input, render};

pub struct Game {
    pub grid: Grid,
}

impl Game {
    pub fn new() -> Self {
        Self { grid: Grid::new() }
    }

    pub fn update(&mut self) {
        let pixel_size = screen_width() / crate::grid::WIDTH as f32;
        input::paint_with_mouse(&mut self.grid, pixel_size);
    }

        pub fn draw(&self) {
        clear_background(BLUE);

        let pixel_size = screen_width() / crate::grid::WIDTH as f32;
        render::draw_grid(&self.grid, pixel_size);
    }
}  