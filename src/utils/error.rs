use thiserror::Error;

#[derive(Error, Debug)]
pub enum PagError {
    #[error("Failed to download file: {0}")]
    DownloadError(String),

    #[error("Failed to extract archive: {0}")]
    ExtractionError(String),

    #[error("Failed to verify signature: {0}")]
    SignatureError(String),

    #[error("Failed to resolve dependencies: {0}")]
    DependencyError(String),

    #[error("Failed to update bootloader: {0}")]
    BootloaderError(String),

    #[error("Failed to backup kernel: {0}")]
    BackupError(String),

    #[error("Failed to run command: {0}")]
    CommandError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Database error: {0}")]
    DatabaseError(String),
}

pub type Result<T> = std::result::Result<T, PagError>;