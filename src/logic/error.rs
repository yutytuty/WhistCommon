#[derive(thiserror::Error, Debug)]
pub enum LogicError {
    #[error("Generic logic error: {0}")]
    Generic(String),
}
