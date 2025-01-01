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

        Ok(Matrix::create_with(rows, cols, table))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_succeed() {
        let matrix_a: Matrix<i64> = Matrix::create_with(2, 2, vec![2, 5, 9, 5]);
        let matrix_b: Matrix<i64> = Matrix::create_with(2, 2, vec![8, 5, 1, 5]);

        assert_eq!(
            Ok(Matrix::create_with(2, 2, vec![10; 4])),
            matrix_a.add(matrix_b)
        );
    }

    #[test]
    fn add_failed() {
        let matrix_a: Matrix<i64> = Matrix::create_with(2, 2, vec![2, 5, 9, 5]);
        let matrix_b: Matrix<i64> = Matrix::create_with(2, 3, vec![8, 5, 1, 5, 4, 2]);

        assert_eq!(
            Err("Error: both matrix most have the same row and column number.".to_string()),
            matrix_a.add(matrix_b)
        );
    }
}
