use std::fmt;

#[derive(PartialEq, Eq, Debug)]
pub enum MatrixOpsError {
    UnMatchedMatricesError,
    SingleValueMatricError,
    EmptyMatrixError,
}

impl std::error::Error for MatrixOpsError {}

impl fmt::Display for MatrixOpsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MatrixOpsError::UnMatchedMatricesError => {
                write!(f, "The matrices are not compatible for the operation")
            }
            MatrixOpsError::SingleValueMatricError => {
                write!(f, "The matrix has only one item")
            }
            MatrixOpsError::EmptyMatrixError => write!(f, "The matrix is empty"),
        }
    }
}
