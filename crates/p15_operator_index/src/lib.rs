use std::{
    ops::{Index, IndexMut},
    usize,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid2D<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Grid2D<T>
where
    T: Clone,
{
    pub fn new(rows: usize, cols: usize, default_value: T) -> Self {
        let data = vec![default_value; rows * cols];
        Grid2D { data, rows, cols }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Convert (row, col) to linear index
    fn linear_index(&self, row: usize, col: usize) -> usize {
        assert!(
            row < self.rows,
            "row {} out of bounds (max {})",
            row,
            self.rows
        );
        assert!(
            col < self.cols,
            "col {} out of bounds (max {})",
            col,
            self.cols
        );
        row * self.cols + col
    }
}

impl<T> Index<(usize, usize)> for Grid2D<T>
where
    T: Clone,
{
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (r, c) = index;
        let idx = self.linear_index(r, c);
        &self.data[idx]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid2D<T>
where
    T: Clone,
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (r, c) = index;
        let idx = self.linear_index(r, c);
        &mut self.data[idx]
    }
}

// --------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_creation() {
        let grid2d = Grid2D::new(3, 4, 0);
        assert_eq!(grid2d.rows(), 3);
        assert_eq!(grid2d.cols(), 4);
    }

    #[test]
    fn test_index_read() {
        let grid2d = Grid2D::new(2, 3, 42);
        assert_eq!(grid2d[(0, 0)], 42);
        assert_eq!(grid2d[(1, 2)], 42);
    }

    #[test]
    #[should_panic(expected = "row 5 out of bounds")]
    fn test_index_out_of_bounds() {
        let grid = Grid2D::new(3, 3, 0);
        let _ = grid[(5, 0)]; // Should panic
    }
}
