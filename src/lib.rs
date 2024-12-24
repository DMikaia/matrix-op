#[allow(unused)]
#[derive(Debug)]
pub struct Matrix {
    rows: u32,
    cols: u32,
    table: Vec<f64>,
}

impl Matrix {
    pub fn new(rows: u32, cols: u32) -> Self {
        Self {
            rows,
            cols,
            table: Vec::with_capacity((rows * cols) as usize),
        }
    }
}
