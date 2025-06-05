// use std::{fs};
// use flate2::read::GzDecoder;
// use zip::{ZipArchive, read::ZipFile};
//
// use super::super::error::FastQError;
//
//
// #[derive(Debug)]
// pub(super) enum CompressionType {
//     Gzip,
//     Bzip2,
//     XZ,
//     Uncompressed,
//     Unknown(String),
// }
//
// impl CompressionType {
//     /// Infers the compression type, only by reading the file extension.
//     /// For example, a ".gz" extension will be parsed as a Gzip-compressed file.
//     pub(super) fn infer_from_filename(filename: &str) -> Self {
//         match filename.rsplit(".").next() {
//             Some("gz") => Self::Gzip,
//             Some("bz2") => Self::Bzip2,
//             Some("xz") => Self::XZ,
//             Some("fastq") | Some("fq") => Self::Uncompressed,
//             Some(other_ext) => Self::Unknown(other_ext.to_string()),
//             None => unreachable!()
//         }
//     }
// }
//
//
// pub(super) fn open_gzip(file: fs::File) -> Result<GzDecoder<fs::File>, FastQError> {
//     Ok(GzDecoder::new(file))
// }
//
//
// pub(super) fn open_zip<'a>(file: fs::File)
//     -> Result<ZipArchive<fs::File>, FastQError>
// {
//     Ok(
//         ZipArchive::new(file)
//             .map_err(
//                 |err| FastQError::DecompressionError("Error while reading bzip archive".into())
//             )?
//     )
// }
//
// pub(super) fn extract_zip_archive(archive: &mut ZipArchive<fs::File>) -> Result<ZipFile, FastQError> {
//     match archive.len() {
//         0 => { return Err(FastQError::DecompressionError("Bzip2 Archive did not contain any files.".into())) },
//         1 => {},
//         _ => { return Err(FastQError::DecompressionError(
//             format!("Bzip2 Archive has {} files. This program only handles single-file bzip2 fastQ archives.", archive.len()).into()
//         )) }
//     }
//
//     let zipped_file = archive.by_index(0)
//         .map_err(
//             |err| FastQError::DecompressionError("Error while accessing file index 0 of bzip archive".into())
//         )?;
//
//     Ok(zipped_file)
// }
