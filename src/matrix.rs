use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Matrix<T: Default + Clone> {
    rows: usize,
    cols: usize,
    table: Vec<T>,
}

impl<T: Default + Clone> Matrix<T> {
    /// This will create a matrix with a table that will contain a default value for each element
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            table: vec![T::default(); rows * cols],
        }
    }

    /// This will create a matrix with an already defined table
    pub fn create_with(rows: usize, cols: usize, table: Vec<T>) -> Self {
        Self { rows, cols, table }
    }

    /// This will return the rows and columns number of the matrix
    pub fn get_size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    /// This will return the index of an element if it is inbounds
    pub fn get_index(&self, (row, col): (usize, usize)) -> Option<usize> {
        if self.in_bounds((row, col)) {
            Some(row * self.cols + col)
        } else {
            None
        }
    }

    /// This will return the element only if the row and column are valid
    pub fn get(&self, (row, col): (usize, usize)) -> Option<&T> {
        if let Some(index) = self.get_index((row, col)) {
            self.table.get(index)
        } else {
            None
        }
    }

    /// This will push a new element to a specific row and column only if they are valid.
    pub fn push(&mut self, (row, col): (usize, usize), value: T) -> Result<(), &'static str> {
        if let Some(index) = self.get_index((row, col)) {
            self.table[index] = value;
            Ok(())
        } else {
            Err("Error: The row and column you have provided are not in range.")
        }
    }

    /// This is used to check whether the row and column are within the matrix interval.
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
            Err("Error: The row and column you have provided are not in range."),
            matrix.push((0, 3), 0.5)
        );
    }
}
