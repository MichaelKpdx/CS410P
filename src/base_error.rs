use crate::*;

//Enum for question base error
#[allow(dead_code)]
#[derive(Debug, thiserror::Error, Serialize)]
pub enum QuestionBaseErr {
    #[error("question already exists: {0}")]
    QuestionExists(String),
    #[error("question base io failed: {0}")]
    QuestionBaseIoError(String),
    #[error("no question")]
    NoQuestion,
    #[error("Question {0} doesn't exist")]
    QuestionDoesNotExist(String),
    #[error("Question paylor unprocessable")]
    QuestionUnprocessable(String),
    #[error("database error: {0}")]
    DatabaseError(String),
}

impl From<std::io::Error> for QuestionBaseErr {
    fn from(e: std::io::Error) -> Self {
        QuestionBaseErr::QuestionBaseIoError(e.to_string())
    }
}

impl From<sqlx::Error> for QuestionBaseErr {
    fn from(e: sqlx::Error) -> Self {
        QuestionBaseErr::DatabaseError(e.to_string())
    }
}

//Struct for Question Base error
#[derive(Debug)]
pub struct QuestionBaseError {
    pub status: StatusCode,
    pub error: QuestionBaseErr,
}
