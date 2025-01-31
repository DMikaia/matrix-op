use matrix_op::Matrix;

fn main() {
    let matrix_a: Matrix<f64> =
        Matrix::create_with(3, 3, vec![3.0, 4.0, 7.0, 2.0, 10.0, 9.0, 2.0, 5.0, 9.0]);
    let matrix_b: Matrix<f64> =
        Matrix::create_with(3, 3, vec![3.0, 1.0, 5.0, 2.0, 9.0, 7.0, 2.0, 5.0, 9.0]);

    println!("Matrix A:\n");
    print!("{}", matrix_a);

    println!("\nMatrix B:\n");
    print!("{}", matrix_b);

    println!("\nTransposing the matrix A:\n");
    print!("{}", matrix_a.transpose());

    println!("\nAdding matrix A and B:\n");
    print!("{}", &matrix_a + &matrix_b);

    println!("\nSubstracting matrix A and B:\n");
    print!("{}", &matrix_a - &matrix_b);

    println!("\nMultiplying matrix A and B:\n");
    print!("{}", &matrix_a * &matrix_b);
}
