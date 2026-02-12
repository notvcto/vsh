use thiserror::Error;

pub type Result<T> = std::result::Result<T, VshError>;

#[derive(Error, Debug)]
pub enum VshError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Execution error: {0}")]
    ExecutionError(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Invalid syntax: {0}")]
    InvalidSyntax(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Unknown command: {0}")]
    UnknownCommand(String),

    #[error("{0}")]
    Other(String),
}

impl VshError {
    pub fn with_suggestion(&self) -> String {
        match self {
            VshError::UnknownCommand(_cmd) => {
                format!(
                    "{}\n\nSuggestion: Type 'help' to see available commands",
                    self
                )
            }
            VshError::FileNotFound(_path) => {
                format!(
                    "{}\n\nSuggestion: Check if the file exists with 'list' command",
                    self
                )
            }
            _ => self.to_string(),
        }
    }
}
