use crate::matrix::Matrix;
use std::ops::Add;

impl<T: Add<Output = T> + Copy> Matrix<T> {
    pub fn add(&self, rhs: Self) -> Result<Matrix<T>, String> {
        if self.get_size() != rhs.get_size() {
            return Err("Error: both matrix most have the same row and column number.".to_string());
        }
        let (rows, cols) = self.get_size();

        let mut table = vec![];
        for row in 0..rows {
            for col in 0..cols {
                if let (Some(&a), Some(&b)) = (self.get((row, col)), rhs.get((row, col))) {
                    table.push(a + b);
                }
            }
        }

        Ok(Matrix::new(rows, cols, table))
    }
}
