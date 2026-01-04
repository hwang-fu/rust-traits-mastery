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

    /// Safe access — returns None if out of bounds
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            Some(&self.data[row * self.cols + col])
        } else {
            None
        }
    }

    /// Safe mutable access
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row < self.rows && col < self.cols {
            Some(&mut self.data[row * self.cols + col])
        } else {
            None
        }
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

impl<T> Index<usize> for Grid2D<T> {
    type Output = [T];
    fn index(&self, row: usize) -> &Self::Output {
        assert!(
            row < self.rows,
            "row {} out of bounds (max {})",
            row,
            self.rows
        );
        let start = row * self.cols;
        let end = start + self.cols;
        &self.data[start..end]
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

impl<T> IndexMut<usize> for Grid2D<T> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        assert!(
            row < self.rows,
            "row {} out of bounds (max {})",
            row,
            self.rows
        );
        let start = row * self.cols;
        let end = start + self.cols;
        &mut self.data[start..end]
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

    #[test]
    fn test_index_mut() {
        let mut grid = Grid2D::new(2, 2, 0);

        grid[(0, 0)] = 1;
        grid[(0, 1)] = 2;
        grid[(1, 0)] = 3;
        grid[(1, 1)] = 4;

        assert_eq!(grid[(0, 0)], 1);
        assert_eq!(grid[(0, 1)], 2);
        assert_eq!(grid[(1, 0)], 3);
        assert_eq!(grid[(1, 1)], 4);
    }

    #[test]
    fn test_modify_existing() {
        let mut grid = Grid2D::new(2, 2, 10);
        grid[(0, 0)] += 5;
        assert_eq!(grid[(0, 0)], 15);
    }

    #[test]
    fn test_index_row() {
        let mut grid = Grid2D::new(2, 3, 0);

        // Set values
        grid[(0, 0)] = 1;
        grid[(0, 1)] = 2;
        grid[(0, 2)] = 3;

        // Get entire row as slice
        let row0: &[i32] = &grid[0];
        assert_eq!(row0, &[1, 2, 3]);

        let row1: &[i32] = &grid[1];
        assert_eq!(row1, &[0, 0, 0]);
    }

    #[test]
    fn test_index_row_mut() {
        let mut grid = Grid2D::new(2, 3, 0);

        // Mutate entire row via slice
        grid[0].copy_from_slice(&[1, 2, 3]);

        assert_eq!(grid[(0, 0)], 1);
        assert_eq!(grid[(0, 1)], 2);
        assert_eq!(grid[(0, 2)], 3);
    }

    #[test]
    fn test_safe_get() {
        let grid = Grid2D::new(2, 2, 42);

        // Valid access
        assert_eq!(grid.get(0, 0), Some(&42));

        // Out of bounds — returns None instead of panicking
        assert_eq!(grid.get(5, 0), None);
        assert_eq!(grid.get(0, 5), None);
    }
}
