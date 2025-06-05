mod fastq;
use fastq::summarize::summarize;
use fastq::file::FastQFile;


use clap::Parser;

#[derive(Parser,Debug)]
#[command(version, about, long_about = None)]
struct Args {
    fname: String,
}


fn main() {
    let args = Args::parse();

    let file = FastQFile::new(args.fname.as_ref()).unwrap();
    if file.is_compressed() {
        println!("Compression: {}", file.compression_type.to_string());
    }

    match summarize(file) {
        Ok(summary) => {
            println!("Number of records: {}", summary.num_records);
            println!("Max. read length: {}", summary.max_record_len);
            println!("Min. read length: {}", summary.min_record_len);
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }
}
