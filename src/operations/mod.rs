pub mod add;
pub mod mul;

use crate::matrix::Matrix;
use std::ops::{Add, Mul};

impl<T: Add<Output = T> + Mul<Output = T> + Copy + Default> Matrix<T> {
    /// This function multiplies two matrices only if the number of columns in the
    /// first matrix is equal to the number of rows in the second matrix.
    pub fn multiply_to(&self, rhs: &Self) -> Result<Matrix<T>, &'static str> {
        let (rows_a, cols_a) = self.get_size();
        let (rows_b, cols_b) = rhs.get_size();

        if cols_a != rows_b {
            return Err(
                "Error: The column of the first matrix most be equal to the row of the second one.",
            );
        }

        let mut table = vec![];
        for row_a in 0..rows_a {
            for col_b in 0..cols_b {
                let mut value = T::default();

                for i in 0..cols_a {
                    if let (Some(&a), Some(&b)) = (self.get(row_a, i), rhs.get(i, col_b)) {
                        value = value + (a * b);
                    }
                }
                table.push(value);
            }
        }

        Ok(Matrix::create_with(rows_a, cols_b, table))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn multiply_to_succeed() {
        let matrix_a: Matrix<f64> = Matrix::create_with(1, 3, vec![3.0, 1.0, 4.0]);
        let matrix_b: Matrix<f64> = Matrix::create_with(3, 2, vec![4.0, 3.0, 2.0, 5.0, 6.0, 8.0]);

        assert_eq!(
            Ok(Matrix::create_with(1, 2, vec![38.0, 46.0])),
            matrix_a.multiply_to(&matrix_b)
        );
    }

    #[test]
    fn multiply_to_failed() {
        let matrix_a: Matrix<i64> = Matrix::create_with(2, 1, vec![2, 5]);
        let matrix_b: Matrix<i64> = Matrix::create_with(2, 3, vec![8, 5, 1, 5, 4, 2]);

        assert_eq!(
            Err(
                "Error: The column of the first matrix most be equal to the row of the second one."
            ),
            matrix_a.multiply_to(&matrix_b)
        );
    }
}
