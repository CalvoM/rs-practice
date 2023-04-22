#[derive(PartialEq, Eq, Debug)]
pub enum MatrixInstance {
    Matrix(Vec<Vec<isize>>),
    Value(isize),
    Empty,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub values: MatrixInstance,
}
impl Matrix {
    pub fn new(rows: usize, cols: usize, values: MatrixInstance) -> Self {
        Self { cols, rows, values }
    }
    pub fn set_values(&mut self, values: MatrixInstance) {
        self.values = values;
    }
}
