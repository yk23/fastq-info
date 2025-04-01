use std::path::PathBuf;
use super::super::error::FastQError;
use super::reader::FastQReader;
use super::compression::*;
use super::validate::*;


#[test]
fn error_on_file_io() {
    let result = FastQReader::from_filename("nonexistant_file.fastq".to_string());
    assert!(result.is_err());
}

#[test]
fn known_compression_format_parse() {
    // See unknown_format_parse() for a for the "UNKNOWN" edge case.
    assert!(matches!(CompressionType::infer_from_filename("nonexistant.fq.gz"), CompressionType::Gzip));
    assert!(matches!(CompressionType::infer_from_filename("nonexistant.fastq.gz"), CompressionType::Gzip));
    assert!(matches!(CompressionType::infer_from_filename("nonexistant.something.gz"), CompressionType::Gzip));
    assert!(matches!(CompressionType::infer_from_filename("nonexistant.gz"), CompressionType::Gzip));

    assert!(matches!(CompressionType::infer_from_filename("nonexistant.fq.bz2"), CompressionType::Bzip2));
    assert!(matches!(CompressionType::infer_from_filename("nonexistant.fastq.bz2"), CompressionType::Bzip2));
    assert!(matches!(CompressionType::infer_from_filename("nonexistant.something.bz2"), CompressionType::Bzip2));
    assert!(matches!(CompressionType::infer_from_filename("nonexistant.bz2"), CompressionType::Bzip2));

    assert!(matches!(CompressionType::infer_from_filename("nonexistant.fq.xz"), CompressionType::XZ));
    assert!(matches!(CompressionType::infer_from_filename("nonexistant.fastq.xz"), CompressionType::XZ));
    assert!(matches!(CompressionType::infer_from_filename("nonexistant.something.xz"), CompressionType::XZ));
    assert!(matches!(CompressionType::infer_from_filename("nonexistant.xz"), CompressionType::XZ));

    assert!(matches!(CompressionType::infer_from_filename("nonexistant.fq"), CompressionType::Uncompressed));
    assert!(matches!(CompressionType::infer_from_filename("nonexistant.fastq"), CompressionType::Uncompressed));
}

#[test]
fn unknown_compression_format_parse() {
    let unk_result = CompressionType::infer_from_filename("nonexistant.fq.foo");
    assert!(matches!(unk_result, CompressionType::Unknown(_)));  // Rust lesson: right-side argument is a wildcard pattern-match.
    if let CompressionType::Unknown(ext) = unk_result {
        assert_eq!(ext, "foo");
    } else {
        panic!("Expected `Unknown` compression type, but got something else: {:?}", unk_result);
    }
}

#[test]
fn compression_validate() {
    let resource_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let fastq_path = resource_dir.join("resources/tests/compressed.fq.gz");
    assert!(fastq_path.exists());
    let filename = fastq_path.to_str().unwrap();
    assert!(validate_gzip_fastq(filename).is_ok());
    assert!(validate_bzip2_fastq(filename).is_err());
    assert!(validate_xz_fastq(filename).is_err());
    assert!(validate_uncompressed_fastq(filename).is_err());

    let fastq_path = resource_dir.join("resources/tests/compressed.fq.bz2");
    assert!(fastq_path.exists());
    let filename = fastq_path.to_str().unwrap();
    assert!(validate_gzip_fastq(filename).is_err());
    assert!(validate_bzip2_fastq(filename).is_ok());
    assert!(validate_xz_fastq(filename).is_err());
    assert!(validate_uncompressed_fastq(filename).is_err());

    let fastq_path = resource_dir.join("resources/tests/compressed.fq.xz");
    assert!(fastq_path.exists());
    let filename = fastq_path.to_str().unwrap();
    assert!(validate_gzip_fastq(filename).is_err());
    assert!(validate_bzip2_fastq(filename).is_err());
    assert!(validate_xz_fastq(filename).is_ok());
    assert!(validate_uncompressed_fastq(filename).is_err());

    let fastq_path = resource_dir.join("resources/tests/short_example.fq");
    assert!(fastq_path.exists());
    let filename = fastq_path.to_str().unwrap();
    assert!(validate_gzip_fastq(filename).is_err());
    assert!(validate_bzip2_fastq(filename).is_err());
    assert!(validate_xz_fastq(filename).is_err());
    assert!(validate_uncompressed_fastq(filename).is_ok());

    let fastq_path = resource_dir.join("resources/tests/bad_fastq.fq");
    assert!(fastq_path.exists());
    let filename = fastq_path.to_str().unwrap();
    let validate_result = validate_uncompressed_fastq(filename);
    assert!(validate_result.is_err());
    assert!(matches!(
            validate_result.unwrap_err(),
            FastQError::WrongFormat { filename: _, malformed_line: _, mal_line_no: _}
        ));
    assert!(matches!(
            validate_uncompressed_fastq("nonexistant.fq").unwrap_err(),
            FastQError::IOError(_)
        ))
}