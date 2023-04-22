pub use self::error::MatrixOpsError;
pub use self::matrix::{Matrix, MatrixInstance};
pub use self::operations::multiply_matrices;
mod error;
mod matrix;
mod operations;
