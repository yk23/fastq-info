use std::fs;
use std::path::PathBuf;
use super::error::FastQError;


#[derive(Debug)]
pub enum CompressionType {
    Gzip,
    Bzip2,
    XZ,
    Uncompressed,
    Unknown(String),
}


impl CompressionType {
    /// Infers the compression type, only by reading the file extension.
    /// For example, a ".gz" extension will be parsed as a Gzip-compressed file.
    pub(super) fn infer_from_filename(filename: &str) -> Self {
        match filename.rsplit(".").next() {
            Some("gz") => Self::Gzip,
            Some("bz2") => Self::Bzip2,
            Some("xz") => Self::XZ,
            Some("fastq") | Some("fq") => Self::Uncompressed,
            Some(other_ext) => Self::Unknown(other_ext.to_string()),
            None => unreachable!()
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Self::Gzip => "GZIP",
            Self::Bzip2 => "BZIP2",
            Self::XZ => "XZ",
            Self::Uncompressed => "UNCOMPRESSED",
            Self::Unknown(_) => "UNKNOWN",
        }
    }
}



/// A struct which represents a FastQ file object, meant to be used by [`FastQReader`].
pub struct FastQFile {
    pub filename: String,
    pub compression_type: CompressionType,
    pub(super) io_file: fs::File,
}


impl FastQFile {
    /// Open the specified file string path as a [`FastQFile`] handle.
    /// Automatically handles compression.
    pub fn new(filename: &str) -> Result<Self, FastQError> {
        let compression = CompressionType::infer_from_filename(filename);
        let inner_file = fs::File::open(filename).map_err(FastQError::from)?;
        let obj = Self {
            filename: filename.to_string(),
            compression_type: compression,
            io_file: inner_file,
        };
        Ok(obj)
    }

    /// Open the specified file string path as a [`FastQFile`] handle.
    /// Automatically handles compression.
    /// Same as [`Self::open`], but takes a PathBuf instead.
    pub fn from_path(file_path: PathBuf) -> Result<Self, FastQError> {
        match file_path.to_str() {
            Some(str_path) => Self::new(str_path),
            None => Err(FastQError::PathError)
        }
    }

    pub fn is_compressed(&self) -> bool {
        !matches!(self.compression_type, CompressionType::Uncompressed)
    }
}