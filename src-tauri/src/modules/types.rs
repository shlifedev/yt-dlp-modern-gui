#[derive(Debug, thiserror::Error, specta::Type, serde::Serialize)]
pub enum AppError {
    #[error("File error: {0}")]
    FileError(String),
    #[error("{0}")]
    Custom(String),
    #[error("Binary not found: {0}")]
    BinaryNotFound(String),
    #[error("Download error: {0}")]
    DownloadError(String),
    #[error("Metadata error: {0}")]
    MetadataError(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
}
