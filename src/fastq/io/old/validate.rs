use std::{fs, io};

use super::super::error::FastQError;
use super::compression::{open_gzip};


// --- These functions validate the actual file compression type by running them through checkers.
pub(super) fn validate_gzip_fastq(filename: &str) -> Result<(), FastQError> {
    let file = fs::File::open(filename).unwrap(); //todo don't panic here.
    let gz_file = open_gzip(file).unwrap(); //todo don't panic here.
    validate_fastq_stream(
        io::BufReader::new(Box::new(gz_file))
    )
}

pub(super) fn validate_bzip2_fastq(filename: &str) -> Result<(), FastQError> {
    unimplemented!()
}

pub(super) fn validate_xz_fastq(filename: &str) -> Result<(), FastQError> {
    unimplemented!()
}

pub(super) fn validate_uncompressed_fastq(filename: &str) -> Result<(), FastQError> {
    let file = fs::File::open(filename).unwrap(); //todo don't panic here.
    let buf: io::BufReader<Box<dyn io::Read>> = io::BufReader::new(Box::from(file));  //todo don't panic here.
    validate_fastq_stream(buf)
}


/// Main implementation. TODO docstring.
fn validate_fastq_stream(reader: io::BufReader<Box<dyn io::Read>>) -> Result<(), FastQError> {
    unimplemented!() // todo write docstring.
}