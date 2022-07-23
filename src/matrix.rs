use std::fmt::Debug;

use crate::error::{UtilsError, UtilsResult};

pub struct Matrix<T> {
    columns: usize,
    rows: usize,
    data: Vec<T>,
}

impl<T: Default + Clone + Debug> Matrix<T> {
    /**
    Constructs a new, empty `Matrix<T>`.

    # Examples

    ```
    use new_york_utils::Matrix;

    let mut matrix: Matrix<f64> = Matrix::new(3, 4);
    ```
     */
    pub fn new(columns: usize, rows: usize) -> Self {
        Matrix {
            columns,
            rows,
            data: vec![T::default(); columns * rows],
        }
    }

    /**
    get item from matrix.

    # Examples

    ```
    use new_york_utils::Matrix;

    let mut matrix: Matrix<f64> = Matrix::new(3, 4);
    let result = matrix.get(1, 2);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), 0.0);
    ```
     */
    pub fn get(&self, column: usize, row: usize) -> UtilsResult<T> {
        let pos = self.get_pos(column, row);

        if pos.is_err() {
            return Err(pos.unwrap_err());
        }

        match self.data.get(pos.unwrap()) {
            None => Err(UtilsError::NotFound),
            Some(val) => Ok(val.clone()),
        }
    }

    /**
    set item to matrix.

    # Examples

    ```
    use new_york_utils::Matrix;

    let mut matrix = Matrix::new(3, 4);
    let result = matrix.set(1, 2, 1.0);
    assert_eq!(result.is_ok(), true);
    assert_eq!(matrix.get(1, 2).unwrap(), 1.0);
    ```
     */
    pub fn set(&mut self, column: usize, row: usize, val: T) -> UtilsResult<()> {
        let pos = self.get_pos(column, row);

        if pos.is_err() {
            return Err(pos.unwrap_err());
        }

        self.data[pos.unwrap()] = val;
        Ok(())
    }

    /**
    get matrix shape

    # Examples

    ```
    use new_york_utils::Matrix;

    let mut matrix: Matrix<f64> = Matrix::new(3, 4);
    assert_eq!(matrix.get_shape(), vec![3,4]);
    ```
     */
    pub fn get_shape(&self) -> Vec<usize> {
        vec![self.columns, self.rows]
    }

    pub fn get_columns(&self) -> usize {
        self.columns
    }

    pub fn get_rows(&self) -> usize {
        self.rows
    }

    /**
    get raw matrix data

    # Examples

    ```
    use new_york_utils::Matrix;

    let mut matrix: Matrix<f64> = Matrix::new(3, 4);
    let data = matrix.get_data();
    assert_eq!(format!("{:?}", data), "Ok([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])");
    ```
     */
    pub fn get_data(&self) -> UtilsResult<Vec<T>> {
        Ok(self.data.to_vec())
    }

    /**
    set raw matrix data

    # Examples

    ```
    use new_york_utils::Matrix;

    let mut matrix: Matrix<i32> = Matrix::new(3, 4);
    let raw_data = vec![1,2,3,4,5,6,7,8,9,10,11,12];
    assert_eq!(matrix.set_data(raw_data).is_ok(), true);
    assert_eq!(format!("{:?}", matrix.get_data()), "Ok([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12])");
    ```
     */
    pub fn set_data(&mut self, data: Vec<T>) -> UtilsResult<()> {
        if data.len() != self.columns * self.rows {
            return Err(UtilsError::WrongSize);
        }

        self.data = data;

        Ok(())
    }

    /**
    transpose matrix

    # Examples

    ```
    use new_york_utils::Matrix;

    let mut matrix: Matrix<i32> = Matrix::new(4, 3);
    let raw_data = vec![1,2,3,4,5,6,7,8,9,10,11,12];
    let _ = matrix.set_data(raw_data);

    let transposed = matrix.transpose();

    assert_eq!(transposed.is_ok(), true);

    let transposed = transposed.unwrap();
    assert_eq!(transposed.get_shape(), vec![3,4]);
    assert_eq!(format!("{:?}", transposed.get_data()), "Ok([1, 5, 9, 2, 6, 10, 3, 7, 11, 4, 8, 12])");
    ```
     */
    pub fn transpose(&self) -> UtilsResult<Matrix<T>> {
        let mut matrix = Matrix::new(self.rows, self.columns);

        for column in 0..self.columns {
            for row in 0..self.rows {
                let val = self.get(column, row);

                if val.is_err() {
                    return Err(val.unwrap_err());
                }

                let _ = matrix.set(row, column, val.unwrap());
            }
        }

        Ok(matrix)
    }

    /**
    slice matrix by row

    # Examples

    ```
    use new_york_utils::Matrix;

    let mut matrix: Matrix<i32> = Matrix::new(4, 3);
    let raw_data = vec![1,2,3,4,5,6,7,8,9,10,11,12];
    let _ = matrix.set_data(raw_data);

    let sliced = matrix.slice(1,2);
    assert_eq!(sliced.is_ok(), true);

    let sliced = sliced.unwrap();
    assert_eq!(sliced.get_shape(), vec![4,1]);
    assert_eq!(format!("{:?}", sliced.get_data()), "Ok([5, 6, 7, 8])");
    ```
     */
    pub fn slice(&self, from: usize, to: usize) -> UtilsResult<Matrix<T>> {
        let delta = to - from;
        let mut matrix = Matrix::new(self.columns, delta);

        for row in 0..delta {
            for column in 0..self.columns {
                let val = self.get(column, delta + row);

                if val.is_err() {
                    return Err(val.unwrap_err());
                }

                let _ = matrix.set(column, row, val.unwrap());
            }
        }

        Ok(matrix)
    }

    fn get_pos(&self, column: usize, row: usize) -> UtilsResult<usize> {
        if column >= self.columns || row >= self.rows {
            return Err(UtilsError::WrongPosition);
        }

        Ok(row * self.columns + column)
    }

    /**
    add row into matrix

    # Examples

    ```
    use new_york_utils::Matrix;

    let mut matrix: Matrix<i32> = Matrix::new(4, 3);
    let raw_data = vec![0,0,0,0,0,0,0,0,0,0,0,0];
    let _ = matrix.set_data(raw_data);

    let add_row = matrix.add_row(1,vec![1,1,1,1]);
    assert_eq!(add_row.is_ok(), true);
    assert_eq!(format!("{:?}", matrix.get_data()), "Ok([0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0])");
    ```
     */
    pub fn add_row(&mut self, row: usize, data: Vec<T>) -> UtilsResult<()> {
        for i in 0..self.columns {
            let _ = self.set(i, row, data[i].clone());
        }

        Ok(())
    }

    /**
    add column into matrix

    # Examples

    ```
    use new_york_utils::Matrix;

    let mut matrix: Matrix<i32> = Matrix::new(4, 3);
    let raw_data = vec![0,0,0,0,0,0,0,0,0,0,0,0];
    let _ = matrix.set_data(raw_data);

    let add_column = matrix.add_column(1,vec![1,1,1]);
    assert_eq!(add_column.is_ok(), true);
    assert_eq!(format!("{:?}", matrix.get_data()), "Ok([0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0])");
    ```
     */
    pub fn add_column(&mut self, column: usize, data: Vec<T>) -> UtilsResult<()> {
        for i in 0..self.rows {
            let _ = self.set(column, i, data[i].clone());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

    #[test]
    fn it_works() {
        let mut matrix = Matrix::new(4, 3);

        let result = matrix.set(1, 2, 1.0);
        assert_eq!(result.is_ok(), true);
        assert_eq!(matrix.get(1, 2).unwrap(), 1.0);

        assert_eq!(matrix.get(3, 3).is_ok(), false);
        assert_eq!(matrix.get(2, 4).is_ok(), false);
    }
}
