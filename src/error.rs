use thiserror::Error;

#[derive(Error, Debug)]
pub enum BiaoError {
    #[error("gh CLI not found: {message}\n\nPlease install GitHub CLI: https://cli.github.com/")]
    GhNotFound { message: String },
    
    #[error("gh CLI error: {message}")]
    GhError { message: String },
    
    #[error("JSON parse error: {message}")]
    ParseError { message: String },
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, BiaoError>;
