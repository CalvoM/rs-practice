use crate::{Matrix, MatrixInstance, MatrixOpsError};
// fn add_matrices() {
//     todo!();
// }

pub fn multiply_matrices(m1: &Matrix, m2: &Matrix) -> Result<Matrix, MatrixOpsError> {
    let is_m1_single_item = m1.rows == 1 && m1.cols == 1;
    let is_m2_single_item = m2.rows == 1 && m2.cols == 1;
    if is_m1_single_item || is_m2_single_item {
        return Err(MatrixOpsError::SingleValueMatricError);
    }
    let is_m1_empty = m1.values == MatrixInstance::Empty;
    let is_m2_empty = m2.values == MatrixInstance::Empty;
    if is_m1_empty || is_m2_empty {
        return Err(MatrixOpsError::EmptyMatrixError);
    }
    if m1.cols != m2.rows {
        return Err(MatrixOpsError::UnMatchedMatricesError);
    }
    let mut result = Matrix::new(m1.rows, m2.cols, MatrixInstance::Empty);
    let mut v1: &Vec<Vec<isize>> = &vec![];
    if let MatrixInstance::Matrix(v) = &m1.values {
        v1 = v;
    };
    let mut v2: &Vec<Vec<isize>> = &vec![];
    if let MatrixInstance::Matrix(v) = &m2.values {
        v2 = v;
    };
    let mut flat_result: Vec<isize> = vec![];
    for row in v1 {
        let col_values: &mut Vec<isize> = &mut vec![];
        let row_values = row;
        let mut v2_col_index = 0;
        while v2_col_index < m2.cols {
            for col in v2 {
                col_values.push(col[v2_col_index]);
            }
            v2_col_index += 1;
            let mut dot_product: isize = 0;
            let mut prod_index = 0;
            while prod_index < row_values.len() {
                let product = row_values[prod_index] * col_values[prod_index];
                dot_product += product;
                prod_index += 1;
            }
            flat_result.push(dot_product);
            col_values.clear();
        }
    }
    let mut final_layout: Vec<Vec<isize>> = vec![];
    let mut final_items_index = 0;
    while final_items_index < flat_result.len() {
        let mut row: Vec<isize> = vec![];
        let mut m = 0;
        while m < m2.cols {
            row.push(flat_result[final_items_index + m]);
            m += 1
        }
        final_layout.push(row.clone());
        final_items_index += m2.rows - 1;
    }
    result.set_values(MatrixInstance::Matrix(final_layout));
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_multiply_matrices() {
        let values = MatrixInstance::Matrix(vec![vec![2, 8, 3], vec![5, 4, 1]]);
        let m1 = Matrix::new(2, 3, values);
        let values = MatrixInstance::Matrix(vec![vec![4, 1], vec![6, 3], vec![2, 4]]);
        let m2 = Matrix::new(3, 2, values);
        let values = MatrixInstance::Matrix(vec![vec![62, 38], vec![46, 21]]);
        let expected = Matrix::new(m1.rows, m2.cols, values);
        let actual = multiply_matrices(&m1, &m2).unwrap();
        assert_eq!(expected.rows, actual.rows);
        assert_eq!(expected.cols, actual.cols);
        assert_eq!(expected.values, actual.values);
    }
    #[test]
    fn test_multiply_matrices_returns_unmatched_errors() {
        let values = MatrixInstance::Matrix(vec![vec![2, 3]]);
        let m1 = Matrix::new(1, 2, values);
        let values = MatrixInstance::Matrix(vec![vec![2], vec![3], vec![4]]);
        let m2 = Matrix::new(3, 1, values);
        let actual = multiply_matrices(&m1, &m2);
        assert_eq!(actual, Err(MatrixOpsError::UnMatchedMatricesError))
    }
    #[test]
    fn test_multiply_matrices_does_not_support_single_item_matrix() {
        let values = MatrixInstance::Matrix(vec![vec![2, 3]]);
        let m1 = Matrix::new(1, 2, values);
        let values = MatrixInstance::Value(12);
        let m2 = Matrix::new(1, 1, values);
        let actual = multiply_matrices(&m1, &m2);
        assert_eq!(actual, Err(MatrixOpsError::SingleValueMatricError));
    }
}
