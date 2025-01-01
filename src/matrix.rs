use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    table: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn create_with(rows: usize, cols: usize, table: Vec<T>) -> Self {
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

    pub fn get(&self, (row, col): (usize, usize)) -> Option<&T> {
        if let Some(index) = self.get_index((row, col)) {
            self.table.get(index)
        } else {
            None
        }
    }

    pub fn push(&mut self, (row, col): (usize, usize), value: T) -> Result<(), String> {
        if let Some(index) = self.get_index((row, col)) {
            self.table[index] = value;
            Ok(())
        } else {
            Err("Error: The row and column you have provided are not in range.".to_string())
        }
    }

    fn in_bounds(&self, (row, col): (usize, usize)) -> bool {
        row < self.rows && col < self.cols
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_size() {
        let matrix: Matrix<f64> = Matrix::create_with(2, 2, vec![0.2, 0.5, 0.9, 0.75]);

        assert_eq!((2usize, 2usize), matrix.get_size());
    }

    #[test]
    fn test_get_index_success() {
        let matrix: Matrix<f64> = Matrix::create_with(2, 2, vec![0.2, 0.5, 0.9, 0.75]);

        assert_eq!(Some(1), matrix.get_index((0, 1)));
    }

    #[test]
    fn test_get_index_failed() {
        let matrix: Matrix<f64> = Matrix::create_with(2, 2, vec![0.2, 0.5, 0.9, 0.75]);

        assert_eq!(None, matrix.get_index((0, 3)));
    }

    #[test]
    fn test_get_success() {
        let matrix: Matrix<f64> = Matrix::create_with(2, 2, vec![0.2, 0.5, 0.9, 0.75]);

        assert_eq!(Some(&0.5), matrix.get((0, 1)));
    }

    #[test]
    fn test_get_failed() {
        let matrix: Matrix<f64> = Matrix::create_with(2, 2, vec![0.2, 0.5, 0.9, 0.75]);

        assert_eq!(None, matrix.get((0, 3)));
    }

    #[test]
    fn test_push_success() {
        let mut matrix: Matrix<f64> = Matrix::create_with(2, 2, vec![0.2, 0.0, 0.9, 0.75]);

        assert_eq!(Ok(()), matrix.push((0, 1), 0.5));
    }

    #[test]
    fn test_push_failed() {
        let mut matrix: Matrix<f64> = Matrix::create_with(2, 2, vec![0.2, 0.0, 0.9, 0.75]);

        assert_eq!(
            Err("Error: The row and column you have provided are not in range.".to_string()),
            matrix.push((0, 3), 0.5)
        );
    }
}
