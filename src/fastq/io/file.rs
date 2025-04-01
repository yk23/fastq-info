use std::fs;
use std::path::PathBuf;
use crate::fastq::error::FastQError;

use super::compression::CompressionType;
use super::validate::{
    validate_uncompressed_fastq,
    validate_bzip2_fastq,
    validate_gzip_fastq,
    validate_xz_fastq
};

/// A struct which represents a FastQ file object, meant to be used by [`FastQReader`].
pub struct FastQFile {
    pub(super) filename: String,
    pub(super) compression_type: CompressionType,
    pub(super) io_file: fs::File,
}

impl FastQFile {
    /// Open the specified file string path as a [`FastQFile`] handle.
    /// Automatically handles compression.
    pub fn open(filename: &str) -> Result<Self, FastQError> {
        let compression = CompressionType::infer_from_filename(filename);
        Self::validate(filename, &compression)?;

        let inner_file = fs::File::open(filename).map_err(FastQError::from)?;
        let obj = Self {
            filename: filename.to_string(),
            compression_type: compression,
            io_file: inner_file,
        };
        return Ok(obj);
    }

    /// Performs validation on the provided filename, by opening it and confirming that the
    /// unpacked text adheres to the FastQ format.
    /// In particular, assuming that the file is not empty, it checks whether line 0 begins
    /// with the "@" symbol, and line 2 begins with the "+" symbol.
    fn validate(filename: &str, compression_type: &CompressionType) -> Result<(), FastQError> {
        match compression_type {
            CompressionType::Gzip => { validate_gzip_fastq(filename)? },
            CompressionType::Bzip2 => { validate_bzip2_fastq(filename)? },
            CompressionType::XZ => { validate_xz_fastq(filename)? },
            CompressionType::Uncompressed => { validate_uncompressed_fastq(filename)? },
            CompressionType::Unknown(ext) => return Err(FastQError::UnknownCompression(ext.into())),
        };
        Ok(())
    }

    /// Open the specified file string path as a [`FastQFile`] handle.
    /// Automatically handles compression.
    /// Same as [`Self::open`], but takes a PathBuf instead.
    fn open_from_path(file_path: PathBuf) -> Result<Self, FastQError> {
        match file_path.to_str() {
            Some(str_path) => Self::open(str_path),
            None => Err(FastQError::PathError)
        }
    }

    fn is_compressed(&self) -> bool {
        !matches!(self.compression_type, CompressionType::Uncompressed)
    }
}