use std::{fs, io};

use super::super::error::FastQError;
use super::compression::{open_gzip, open_zip};


// --- These functions validate the actual file compression type by running them through checkers.
pub(super) fn validate_gzip_fastq(filename: &str) -> Result<(), FastQError> {
    let file = fs::File::open(filename).unwrap(); //todo don't panic here.
    let buf = open_gzip(file).unwrap(); //todo don't panic here.
    validate_fastq_stream(buf)
}

pub(super) fn validate_bzip2_fastq(filename: &str) -> Result<(), FastQError> {
    unimplemented!()
}

pub(super) fn validate_xz_fastq(filename: &str) -> Result<(), FastQError> {
    unimplemented!()
}

pub(super) fn validate_uncompressed_fastq(filename: &str) -> Result<(), FastQError> {
    let file = fs::File::open(filename).unwrap(); //todo don't panic here.
    let buf = Box::from(io::BufReader::new(file));  //todo don't panic here.
    validate_fastq_stream(buf)
}


/// Main implementation. TODO docstring.
fn validate_fastq_stream(reader: Box<dyn io::BufRead>) -> Result<(), FastQError> {
    unimplemented!() // todo write docstring.
}