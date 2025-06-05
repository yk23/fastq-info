use std::io::{BufReader, Read};
use zip::CompressionMethod;

use super::{
    super::error::FastQError,
    file::FastQFile,
    // compression::{CompressionType, open_gzip},
};


pub fn bufreader_from_file(file: FastQFile) -> BufReader<dyn io::BufRead> {
    Self { file }
    // let reader = match file.compression_type {
    //     CompressionType::Gzip => FastQReader {
    //         buf_reader: BufReader::new(
    //             Box::new(open_gzip(file.io_file)?))
    //     },
    //     CompressionType::Bzip2 => FastQReader {
    //         buf_reader: BufReader::new(
    //             Box::new(open_zip(file.io_file, CompressionMethod::BZIP2)?))
    //     },
    //     CompressionType::XZ => FastQReader {
    //         buf_reader: BufReader::new(
    //             Box::new(open_zip(file.io_file, CompressionMethod::XZ)?))
    //     },
    //     CompressionType::Uncompressed => FastQReader {
    //         buf_reader: BufReader::new(
    //             Box::new(file.io_file))
    //     },
    //     CompressionType::Unknown(ext) => {
    //         return Err(FastQError::UnknownCompression(ext))
    //     },
    // };
    // Ok(reader)
}