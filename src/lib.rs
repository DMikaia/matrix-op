#[derive(Debug)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    table: Vec<f64>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize, table: Vec<f64>) -> Self {
        Self { rows, cols, table }
    }

    pub fn in_bounds(&self, (i, j): (usize, usize)) -> bool {
        i < self.rows && j < self.cols
    }

    pub fn get_index(&self, (i, j): (usize, usize)) -> usize {
        i * self.cols + j
    }

    pub fn display_matrix(&self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                print!(
                    "{}{}",
                    self.table[self.get_index((i, j))],
                    if j + 1 == self.cols { "\n" } else { " " }
                );
            }
        }
    }
}
