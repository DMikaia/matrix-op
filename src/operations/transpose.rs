use crate::Matrix;

impl<T: Default + Clone> Matrix<T> {
    pub fn transpose(&self) -> Matrix<T> {
        let (rows, cols) = self.get_size();
        let mut table: Vec<T> = vec![T::default(); rows * cols];

        for row in 0..rows {
            for col in 0..cols {
                if let Some(value) = self.get(row, col) {
                    table[col * rows + row] = value.clone();
                }
            }
        }

        Matrix::create_with(cols, rows, table)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn transpose() {
        let input = Matrix::create_with(2, 3, vec![1, 2, 3, 4, 5, 6]);

        assert_eq!(
            Matrix::create_with(3, 2, vec![1, 4, 2, 5, 3, 6]),
            input.transpose()
        );
    }
}
