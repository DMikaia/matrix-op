# matrix-op

This project is a small project used for matrix operation and manipulation.

## Features

- Display:

```rust
let matrix_a: Matrix<f64> = Matrix::create_with(2, 2, vec![3.0, 4.0, 7.0, 2.0]);

print!("{}", matrix_a);
```

- Addition:

```rust
let matrix_a: Matrix<f64> = Matrix::create_with(2, 2, vec![3.0, 4.0, 7.0, 2.0]);
let matrix_b: Matrix<f64> = Matrix::create_with(3, 3, vec![3.0, 1.0, 5.0, 2.0]);

print!("{}", matrix_a + matrix_b);
```

- Substraction:

```rust
let matrix_a: Matrix<f64> = Matrix::create_with(2, 2, vec![3.0, 4.0, 7.0, 2.0]);
let matrix_b: Matrix<f64> = Matrix::create_with(3, 3, vec![3.0, 1.0, 5.0, 2.0]);

print!("{}", matrix_a - matrix_b);
```

- Multiplication:

```rust
let matrix_a: Matrix<f64> = Matrix::create_with(2, 2, vec![3.0, 4.0, 7.0, 2.0]);
let matrix_b: Matrix<f64> = Matrix::create_with(3, 3, vec![3.0, 1.0, 5.0, 2.0]);

print!("{}", matrix_a * matrix_b);
```

- Transpose:

```rust
let matrix_a: Matrix<f64> = Matrix::create_with(2, 2, vec![3.0, 4.0, 7.0, 2.0]);

print!("{}", matrix_a.transpose());
```
