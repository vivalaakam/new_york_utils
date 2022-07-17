#[derive(Debug)]
pub enum UtilsError {
    WrongPosition,
    NotFound,
}

pub type UtilsResult<T> = std::result::Result<T, UtilsError>;
