use matrix_op::{display_matrix, Matrix};

fn main() {
    let matrix_a: Matrix<f64> =
        Matrix::create_with(3, 3, vec![3.0, 4.0, 7.0, 2.0, 5.0, 9.0, 2.0, 5.0, 9.0]);
    let matrix_b: Matrix<f64> =
        Matrix::create_with(3, 3, vec![3.0, 1.0, 5.0, 6.0, 9.0, 7.0, 2.0, 5.0, 9.0]);

    println!("Matrix A:\n");
    display_matrix(&matrix_a);

    println!("\nMatrix B:\n");
    display_matrix(&matrix_b);

    println!("\nTransposing the matrix A:\n");
    let matrix_transpose = matrix_a.transpose();
    display_matrix(&matrix_transpose);

    println!("\nAdding matrix A and B:\n");
    let matrix_add = &matrix_a + &matrix_b;
    display_matrix(&matrix_add);

    println!("\nSubstracting matrix A and B:\n");
    let matrix_sub = &matrix_a - &matrix_b;
    display_matrix(&matrix_sub);

    println!("\nMultiplying matrix A and B:\n");
    let matrix_mul = &matrix_a * &matrix_b;
    display_matrix(&matrix_mul);
}
