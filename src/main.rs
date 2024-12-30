use matrix_op::{display_matrix, Matrix};

fn main() {
    let matrix_a: Matrix<f64> =
        Matrix::new(3, 3, vec![0.2, 0.5, 0.9, 0.75, 0.6, 0.3, 0.4, 0.2, 0.1]);
    let matrix_b: Matrix<f64> =
        Matrix::new(3, 3, vec![0.8, 0.5, 0.1, 0.25, 0.4, 0.7, 0.6, 0.8, 0.9]);

    display_matrix(&matrix_a);

    println!();

    display_matrix(&matrix_b);

    if let Ok(matrix_c) = matrix_a.add(matrix_b) {
        println!();

        display_matrix(&matrix_c);
    }
}
