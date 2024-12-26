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

    pub fn in_bounds(&self, (row, col): (usize, usize)) -> bool {
        row < self.rows && col < self.cols
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn get_index(&self, (row, col): (usize, usize)) -> Option<usize> {
        if !self.in_bounds((row, col)) {
            None
        } else {
            Some(row * self.cols + col)
        }
    }

    pub fn display_matrix(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                if let Some(index) = self.get_index((row, col)) {
                    print!(
                        "{}{}",
                        self.table[index],
                        if col + 1 == self.cols { "\n" } else { " " }
                    );
                }
            }
        }
    }
}
