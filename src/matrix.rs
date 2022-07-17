use crate::error::{UtilsError, UtilsResult};

pub struct Matrix<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: Default + Clone> Matrix<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Matrix {
            width,
            height,
            data: vec![T::default(); width * height],
        }
    }

    pub fn get(&self, w: usize, h: usize) -> UtilsResult<T> {
        let pos = self.get_pos(w, h);

        if pos.is_err() {
            return Err(pos.unwrap_err());
        }

        match self.data.get(pos.unwrap()) {
            None => Err(UtilsError::NotFound),
            Some(val) => Ok(val.clone())
        }
    }

    pub fn set(&mut self, w: usize, h: usize, val: T) -> UtilsResult<()> {
        let pos = self.get_pos(w, h);

        if pos.is_err() {
            return Err(pos.unwrap_err());
        }

        self.data[pos.unwrap()] = val;
        Ok(())
    }

    fn get_pos(&self, w: usize, h: usize) -> UtilsResult<usize> {
        if w >= self.width || h >= self.height {
            return Err(UtilsError::WrongPosition);
        }

        Ok(w * self.height + h)
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

    #[test]
    fn it_works() {
        let mut matrix = Matrix::new(3, 4);

        let result = matrix.set(1, 2, 1.0);
        assert_eq!(result.is_ok(), true);
        assert_eq!(matrix.get(1, 2).unwrap(), 1.0);

        let _ = matrix.set(2, 3, 2.5);
        assert_eq!(result.is_ok(), true);
        assert_eq!(matrix.get(2, 3).unwrap(), 2.5);

        assert_eq!(matrix.get(3, 3).is_ok(), false);
        assert_eq!(matrix.get(2, 4).is_ok(), false);
    }
}
