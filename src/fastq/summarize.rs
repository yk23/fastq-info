use std::io::{BufReader};
use super::{
    error::FastQError,
    file::{FastQFile, CompressionType},
    reader::FastQReader,
};


pub struct FastQSummary {
    pub num_records: usize,
    pub min_record_len: usize,
    pub max_record_len: usize,
}

fn summarize_uncompressed(file: FastQFile) -> Result<FastQSummary, FastQError> {
    let reader = FastQReader::new(
        BufReader::new(file.io_file)
    );
    summarize_stream(reader)
}

fn summarize_stream(reader: FastQReader<BufReader<std::fs::File>>) -> Result<FastQSummary, FastQError> {
    let mut num_records: usize = 0;
    let mut min_record_len: usize = usize::MAX;
    let mut max_record_len: usize = 0;

    for record_item in reader {
        match record_item {
            Ok(record) => {
                num_records += 1;
                min_record_len = std::cmp::min(min_record_len, record.nucleotides.len());
                max_record_len = std::cmp::max(max_record_len, record.nucleotides.len());
            }
            Err(e) => {
                return Err(e)
            }
        }
    }

    Ok(FastQSummary {
        num_records,
        min_record_len,
        max_record_len,
    })
}

pub fn summarize(file: FastQFile) -> Result<FastQSummary, FastQError> {
    match file.compression_type {
        CompressionType::Gzip => panic!("summarize(): gzip not implemented!"),
        CompressionType::Bzip2 => panic!("summarize(): bzip2 not implemented!"),
        CompressionType::XZ => panic!("summarize(): xz not implemented!"),
        CompressionType::Uncompressed => summarize_uncompressed(file),
        CompressionType::Unknown(extension) => Err(FastQError::UnknownCompression(extension)),
    }
}