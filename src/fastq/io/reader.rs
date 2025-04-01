use std::io;
use zip::CompressionMethod;

use super::{
    super::error::FastQError,
    compression::{CompressionType, open_zip, open_gzip},
    file::FastQFile,
};


/// A buffered FastQ reader.
/// Encapsulates a BufReader object, consuming 4 lines at a time.
/// Implements an iterator, which consumes four lines at a time to parse FASTQ sequence records, one by one.
pub struct FastQReader {
    // Rust lesson: Since I want to specify a type which implements a trait, and not any particular
    // BufReader<FileType> (potentially with lifetimes that I don't want to explicitly set), I need
    // to tell the compiler to use a *pointer* to something which implements the trait io::BufRead.
    // "Box<dyn io::BufRead>" accomplishes this by dynamic dispatch.
    buf_reader: Box<dyn io::BufRead>,
}

impl FastQReader {
    /// Create a new [`FastQReader`] object.
    /// Returns a [`FastQError::IOError`] if IO error occurs.
    pub fn from_filename(filename: String) -> Result<Self, FastQError> {
        let file = FastQFile::open(filename.as_str())?;
        Self::from_file(file)
    }
    pub fn from_file(file: FastQFile) -> Result<Self, FastQError> {
        match file.compression_type {
            CompressionType::Gzip => { Ok(FastQReader { buf_reader: open_gzip(file.io_file)? }) },
            CompressionType::Bzip2 => { Ok(FastQReader { buf_reader: open_zip(file.io_file, CompressionMethod::BZIP2)? }) }
            CompressionType::XZ => { Ok(FastQReader { buf_reader: open_zip(file.io_file, CompressionMethod::XZ)? }) },
            CompressionType::Unknown(ext) => { Err(FastQError::UnknownCompression(ext)) },
            CompressionType::Uncompressed => Ok(FastQReader { buf_reader: Box::from(io::BufReader::new(file.io_file)) })
        }
    }
}