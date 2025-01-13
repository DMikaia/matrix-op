use crate::matrix::Matrix;
use std::ops::{Add, Mul};

impl<'a, 'b, T: Add<Output = T> + Mul<Output = T> + Copy + Default> Mul<&'b Matrix<T>>
    for &'a Matrix<T>
{
    type Output = Matrix<T>;

    fn mul(self, rhs: &'b Matrix<T>) -> Self::Output {
        let (rows_a, cols_a) = self.get_size();
        let (rows_b, cols_b) = rhs.get_size();

        assert_eq!(
            cols_a, rows_b,
            "The column of the first matrix most be equal to the row of the second one."
        );

        let mut table = Vec::new();
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

        Matrix::create_with(rows_a, cols_b, table)
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy + Default> Mul for Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        let (rows_a, cols_a) = self.get_size();
        let (rows_b, cols_b) = rhs.get_size();

        assert_eq!(
            cols_a, rows_b,
            "The column of the first matrix most be equal to the row of the second one."
        );

        let mut table = Vec::new();
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

        Matrix::create_with(rows_a, cols_b, table)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mul_consume() {
        let matrix_a: Matrix<f64> = Matrix::create_with(1, 3, vec![3.0, 1.0, 4.0]);
        let matrix_b: Matrix<f64> = Matrix::create_with(3, 2, vec![4.0, 3.0, 2.0, 5.0, 6.0, 8.0]);

        assert_eq!(
            Matrix::create_with(1, 2, vec![38.0, 46.0]),
            matrix_a * matrix_b
        );
    }

    #[test]
    fn mul_ref() {
        let matrix_a: Matrix<f64> = Matrix::create_with(1, 3, vec![3.0, 1.0, 4.0]);
        let matrix_b: Matrix<f64> = Matrix::create_with(3, 2, vec![4.0, 3.0, 2.0, 5.0, 6.0, 8.0]);

        assert_eq!(
            Matrix::create_with(1, 2, vec![38.0, 46.0]),
            &matrix_a * &matrix_b
        );
    }
}
