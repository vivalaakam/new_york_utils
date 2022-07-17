#[derive(Debug)]
pub enum UtilsError {
    WrongPosition,
    NotFound,
    WrongSize,
}

pub type UtilsResult<T> = std::result::Result<T, UtilsError>;
