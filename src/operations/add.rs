use crate::matrix::Matrix;
use std::ops::Add;

impl<'a, 'b, T: Add<Output = T> + Copy + Default> Add<&'b Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, rhs: &'b Matrix<T>) -> Self::Output {
        assert_eq!(
            self.get_size(),
            rhs.get_size(),
            "Both matrix should have the same row and column number."
        );

        let (rows, cols) = self.get_size();

        let mut result: Matrix<T> = Matrix::new(rows, cols);

        for row in 0..rows {
            for col in 0..cols {
                if let (Some(&a), Some(&b)) = (self.get(row, col), rhs.get(row, col)) {
                    result.push((row, col), a + b).unwrap();
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_consume() {
        let matrix_a: Matrix<i64> = Matrix::create_with(2, 2, vec![2, 5, 9, 5]);
        let matrix_b: Matrix<i64> = Matrix::create_with(2, 2, vec![8, 5, 1, 5]);

        assert_eq!(Matrix::create_with(2, 2, vec![10; 4]), matrix_a + matrix_b);
    }

    #[test]
    fn add_ref() {
        let matrix_a: Matrix<i64> = Matrix::create_with(2, 2, vec![2, 5, 9, 5]);
        let matrix_b: Matrix<i64> = Matrix::create_with(2, 2, vec![8, 5, 1, 5]);

        assert_eq!(Matrix::create_with(2, 2, vec![10; 4]), matrix_a + matrix_b);
    }
}

impl<T: Add<Output = T> + Copy + Default> Add for Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(
            self.get_size(),
            rhs.get_size(),
            "Both matrix should have the same row and column number."
        );

        let (rows, cols) = self.get_size();

        let mut result: Matrix<T> = Matrix::new(rows, cols);

        for row in 0..rows {
            for col in 0..cols {
                if let (Some(&a), Some(&b)) = (self.get(row, col), rhs.get(row, col)) {
                    result.push((row, col), a + b).unwrap();
                }
            }
        }

        result
    }
}
