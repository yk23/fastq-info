use std::io;
use thiserror::Error;

/// Custom error type.
/// The definition and all implementation of "From" go here.
#[derive(Error, Debug)]
pub enum FastQError {
    #[error("Malformed path.")]
    PathError,
    
    #[error("Could not open file.")]
    IOError(#[from] io::Error),

    #[error("Unsupported compression format `{0}`.")]
    UnknownCompression(String),

    #[error("Decompression error. Reason: {0}")]
    DecompressionError(String),

    #[error("File is not a properly formatted fastQ file. [Line {mal_line_no}: {malformed_line}]")]
    WrongFormat { malformed_line: String, mal_line_no: usize },

    #[error("Error while reading file. Reason: {0}")]
    FileReadError(String),
}