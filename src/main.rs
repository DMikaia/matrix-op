use matrix_op::Matrix;

fn main() {
    let matrix: Matrix<f64> = Matrix::new(3, 3, vec![0.2, 0.5, 0.9, 0.75, 0.6, 0.3, 0.4, 0.2, 0.1]);
    matrix.display_matrix();
}
