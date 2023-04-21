#[derive(Debug)]
pub enum UtilsError {
    WrongPosition,
    NotFound,
    WrongSize,
    WrongColumnSize,
}

pub type UtilsResult<T> = std::result::Result<T, UtilsError>;
