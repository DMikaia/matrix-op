use crate::matrix::Matrix;
use std::ops::Sub;

fn sub_matrices<T: Sub<Output = T> + Copy + Default>(
    lhs: &Matrix<T>,
    rhs: &Matrix<T>,
) -> Matrix<T> {
    assert_eq!(
        lhs.get_size(),
        rhs.get_size(),
        "Both matrix should have the same row and column number."
    );

    let (rows, cols) = lhs.get_size();

    let mut result: Matrix<T> = Matrix::new(rows, cols);

    for row in 0..rows {
        for col in 0..cols {
            if let (Some(&a), Some(&b)) = (lhs.get(row, col), rhs.get(row, col)) {
                result.push((row, col), a - b).unwrap();
            }
        }
    }

    result
}

impl<'a, 'b, T: Sub<Output = T> + Copy + Default> Sub<&'b Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;

    fn sub(self, rhs: &'b Matrix<T>) -> Self::Output {
        sub_matrices(self, rhs)
    }
}

impl<T: Sub<Output = T> + Copy + Default> Sub for Matrix<T> {
    type Output = Matrix<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        sub_matrices(&self, &rhs)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sub_consume() {
        let matrix_a: Matrix<i64> = Matrix::create_with(2, 2, vec![8, 5, 1, 5]);
        let matrix_b: Matrix<i64> = Matrix::create_with(2, 2, vec![8, 5, 1, 5]);

        assert_eq!(Matrix::create_with(2, 2, vec![0; 4]), matrix_a - matrix_b);
    }

    #[test]
    fn test_sub_ref() {
        let matrix_a: Matrix<i64> = Matrix::create_with(2, 2, vec![8, 5, 1, 5]);
        let matrix_b: Matrix<i64> = Matrix::create_with(2, 2, vec![8, 5, 1, 5]);

        assert_eq!(Matrix::create_with(2, 2, vec![0; 4]), &matrix_a - &matrix_b);
    }
}
