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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_grid_has_correct_size() {
        let grid = Grid::new();
        assert_eq!(grid.cells.len(), WIDTH * HEIGHT);
    }

    #[test]
    fn index_is_row_major() {
        assert_eq!(Grid::index(0, 0), 0);
        assert_eq!(Grid::index(1, 0), 1);
        assert_eq!(Grid::index(0, 1), WIDTH);
        assert_eq!(Grid::index(WIDTH - 1, HEIGHT - 1), WIDTH * HEIGHT - 1);
    }

    #[test]
    fn set_and_get_cell() {
        let mut grid = Grid::new();

        // Set a value and read it back
        grid.set_cell(5, 7, 42);
        assert_eq!(grid.get_cell(5, 7), 42);

        // Overwrite the same cell
        grid.set_cell(5, 7, -1);
        assert_eq!(grid.get_cell(5, 7), -1);

        // Ensure other cells remain unchanged (still the default 0)
        assert_eq!(grid.get_cell(0, 0), 0);
        assert_eq!(grid.get_cell(WIDTH - 1, HEIGHT - 1), 0);
    }


    #[test]
    fn in_bounds_works() {
        // Corners should be in bounds
        assert!(Grid::in_bounds(0, 0));
        assert!(Grid::in_bounds(WIDTH - 1, HEIGHT - 1));

        // Just outside should be out of bounds
        assert!(!Grid::in_bounds(WIDTH, 0));
        assert!(!Grid::in_bounds(0, HEIGHT));
        assert!(!Grid::in_bounds(WIDTH, HEIGHT));
    }

    #[test]
    #[should_panic]
    fn get_cell_panics_out_of_bounds() {
        let grid = Grid::new();
        // This should panic because it indexes past the end of the vector
        let _ = grid.get_cell(WIDTH, HEIGHT);
    }

}
