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
}