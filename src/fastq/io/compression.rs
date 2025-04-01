use std::{fs, io};
use flate2::read::GzDecoder;
use zip::{CompressionMethod, ZipArchive};

use super::super::error::FastQError;


#[derive(Debug)]
pub(super) enum CompressionType {
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
}


pub(super) fn open_gzip(file: fs::File) -> Result<Box<dyn io::BufRead>, FastQError> {
    let decoder = GzDecoder::new(file);
    Ok(Box::from(io::BufReader::new(decoder)))
}


pub(super) fn open_zip(file: fs::File, expected_format: CompressionMethod) -> Result<Box<dyn io::BufRead>, FastQError> {
    let mut archive = ZipArchive::new(file)
        .map_err(
            |err| FastQError::DecompressionError("Error while reading bzip archive".into())
        )?;

    match archive.len() {
        0 => { return Err(FastQError::DecompressionError("Bzip2 Archive did not contain any files.".into())) },
        1 => {},
        _ => { return Err(FastQError::DecompressionError(
            format!("Bzip2 Archive has {} files. This program only handles single-file bzip2 fastQ archives.", archive.len()).into()
        )) }
    }

    let file = archive.by_index(0)
        .map_err(
            |err| FastQError::DecompressionError("Error while accessing file index 0 of bzip archive".into())
        )?;

    let compression_method = file.compression();
    if compression_method != expected_format {
        return Err(FastQError::DecompressionError(
            format!("Expected {:?} archive, but got something else: {:?}", expected_format, compression_method).into()
        ))
    }

    Ok(Box::from(io::BufReader::new(file)))
}
