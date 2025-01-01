mod matrix;
mod operations;

pub use matrix::*;
use std::fmt::Display;

pub fn display_matrix<T: Default + Clone + Display>(matrix: &Matrix<T>) {
    let (rows, cols) = matrix.get_size();

    for row in 0..rows {
        for col in 0..cols {
            if let Some(value) = matrix.get((row, col)) {
                print!("{}{}", value, if col + 1 == cols { "\n" } else { " " });
            }
        }
    }
}
