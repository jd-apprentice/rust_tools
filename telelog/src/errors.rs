use std::num::ParseIntError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    IO(#[from] std::io::Error),
    #[error("There was a failure handling the settings")]
    Settings,
    #[error("There was a failure parsing the settings: {0}")]
    Parser(String),
    #[error("There was a failure parsing the number: {0}")]
    ParseNumber(#[from] ParseIntError),
    #[error("There was a failure sending the request: {0}")]
    Request(#[from] reqwest::Error),
}

