use matrix_op::{display_matrix, Matrix};

fn main() {
    let matrix_a: Matrix<f64> =
        Matrix::create_with(3, 3, vec![3.0, 4.0, 7.0, 2.0, 5.0, 9.0, 2.0, 5.0, 9.0]);
    let matrix_b: Matrix<f64> =
        Matrix::create_with(3, 3, vec![3.0, 1.0, 5.0, 6.0, 9.0, 7.0, 2.0, 5.0, 9.0]);

    display_matrix(&matrix_a);

    println!();

    display_matrix(&matrix_b);

    if let Ok(matrix_c) = matrix_a.add_to(&matrix_b) {
        println!();

        display_matrix(&matrix_c);
    }

    if let Ok(matrix_c) = matrix_a.multiply_to(&matrix_b) {
        println!();

        display_matrix(&matrix_c);
    }
}
