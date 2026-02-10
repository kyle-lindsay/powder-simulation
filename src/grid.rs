pub const WIDTH : usize = 32;
pub const HEIGHT : usize = 32;

pub struct Grid {
    pub cells: Vec<i32>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            cells: vec![0; WIDTH * HEIGHT],
        }
    }

    #[inline]
    pub fn index(x: usize, y: usize) -> usize {
        y * WIDTH + x
    }

    #[inline]
    pub fn in_bounds(x: usize, y: usize) -> bool {
        x < WIDTH && y < HEIGHT
    }

    pub fn set_cell(&mut self, x: usize, y: usize, value: i32) {
        let i = Self::index(x, y);
        self.cells[i] = value;
    }

    pub fn get_cell(&self, x: usize, y: usize) -> i32 {
        self.cells[Self::index(x, y)]
    }
}
