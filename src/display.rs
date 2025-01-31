use crate::Matrix;
use std::fmt::Display;

impl<T: Display + Default + Clone> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (rows, cols) = self.get_size();

        for row in 0..rows {
            for col in 0..cols {
                if let Some(value) = self.get(row, col) {
                    write!(f, "{}{}", value, if col + 1 == cols { "\n" } else { "  " })?;
                }
            }
        }

        Ok(())
    }
}
