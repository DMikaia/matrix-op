use crate::matrix::Matrix;
use std::ops::{Add, Mul};

fn multiply_matrices<T: Add<Output = T> + Mul<Output = T> + Copy + Default>(
    lhs: &Matrix<T>,
    rhs: &Matrix<T>,
) -> Matrix<T> {
    let (rows_a, cols_a) = lhs.get_size();
    let (rows_b, cols_b) = rhs.get_size();

    assert_eq!(
        cols_a, rows_b,
        "The column of the first matrix most be equal to the row of the second one."
    );

    let mut table: Vec<T> = Vec::new();
    for row_a in 0..rows_a {
        for col_b in 0..cols_b {
            let mut value = T::default();

            for i in 0..cols_a {
                if let (Some(&a), Some(&b)) = (lhs.get(row_a, i), rhs.get(i, col_b)) {
                    value = value + (a * b);
                }
            }
            table.push(value);
        }
    }

    Matrix::create_with(rows_a, cols_b, table)
}

impl<'a, 'b, T: Add<Output = T> + Mul<Output = T> + Copy + Default> Mul<&'b Matrix<T>>
    for &'a Matrix<T>
{
    type Output = Matrix<T>;

    fn mul(self, rhs: &'b Matrix<T>) -> Self::Output {
        multiply_matrices(self, rhs)
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy + Default> Mul for Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        multiply_matrices(&self, &rhs)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn test_mul_panic() {
        let matrix_a: Matrix<i64> = Matrix::create_with(1, 1, vec![9, 1]);
        let matrix_b: Matrix<i64> = Matrix::create_with(2, 2, vec![8, 5, 1, 5]);

        let _ = matrix_a * matrix_b;
    }

    #[test]
    fn test_mul_consume() {
        let matrix_a: Matrix<f64> = Matrix::create_with(1, 3, vec![3.0, 1.0, 4.0]);
        let matrix_b: Matrix<f64> = Matrix::create_with(3, 2, vec![4.0, 3.0, 2.0, 5.0, 6.0, 8.0]);

        assert_eq!(
            Matrix::create_with(1, 2, vec![38.0, 46.0]),
            matrix_a * matrix_b
        );
    }

    #[test]
    fn test_mul_ref() {
        let matrix_a: Matrix<f64> = Matrix::create_with(1, 3, vec![3.0, 1.0, 4.0]);
        let matrix_b: Matrix<f64> = Matrix::create_with(3, 2, vec![4.0, 3.0, 2.0, 5.0, 6.0, 8.0]);

        assert_eq!(
            Matrix::create_with(1, 2, vec![38.0, 46.0]),
            &matrix_a * &matrix_b
        );
    }
}
