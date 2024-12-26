use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct Matrix<T>
where
    T: Display,
{
    rows: usize,
    cols: usize,
    pub table: Vec<T>,
}

impl<T: Display> Matrix<T> {
    pub fn new(rows: usize, cols: usize, table: Vec<T>) -> Self {
        Self { rows, cols, table }
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn get_index(&self, (row, col): (usize, usize)) -> Option<usize> {
        if self.in_bounds((row, col)) {
            Some(row * self.cols + col)
        } else {
            None
        }
    }

    fn in_bounds(&self, (row, col): (usize, usize)) -> bool {
        row < self.rows && col < self.cols
    }
}
