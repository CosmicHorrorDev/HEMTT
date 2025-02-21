use hemtt_error::thiserror;

#[derive(thiserror::Error, Debug)]
/// Error type for the PBO writer/reader
pub enum Error {
    #[error("IO Error: {0}")]
    /// [std::io::Error]
    Io(#[from] std::io::Error),

    #[error("HEMTT does not support the encountered PBO Mime type: {0}")]
    /// HEMTT does not support the encountered PBO Mime type
    UnsupportedMime(u32),
    #[error("Unexpected data after PBO checksum")]
    /// Unexpected data after PBO checksum
    UnexpectedDataAfterChecksum,
    #[error("File is too large for PBO format")]
    /// File is too large for PBO format
    FileTooLarge,
    #[error("Invalid prefix: {0}")]
    /// Invalid prefix
    InvalidPrefix(String),
}
