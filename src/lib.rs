mod matrix;

pub use matrix::*;
use std::fmt::Display;

pub fn display_matrix<T: Display>(matrix: &Matrix<T>) {
    let (rows, cols) = matrix.get_size();

    for row in 0..rows {
        for col in 0..cols {
            if let Some(index) = matrix.get_index((row, col)) {
                print!(
                    "{}{}",
                    matrix.table[index],
                    if col + 1 == cols { "\n" } else { " " }
                );
            }
        }
    }
}
