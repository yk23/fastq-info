use std::io::BufRead;
use crate::fastq::error::FastQError;

pub struct FastQRecord {
    pub identifier: String,
    pub nucleotides: String,
    pub quality: String,
}

pub struct FastQReader<R: BufRead> {
    records_read: usize,
    lines_read: usize,
    reader: R,
}

impl<R: BufRead> FastQReader<R> {
    pub fn new(reader: R) -> Self {
        FastQReader { records_read: 0, lines_read: 0, reader }
    }
}

impl<R: BufRead> Iterator for FastQReader<R> {
    type Item = Result<FastQRecord, FastQError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut lines = Vec::with_capacity(4);

        // read the next four lines.
        for _ in 0..4 {
            let mut buf = String::new();
            self.lines_read += 1;
            match self.reader.read_line(&mut buf) {
                Ok(0) => return None,
                Ok(_) => lines.push(buf.trim_end().to_string()),
                Err(e) => return Some(Err(FastQError::FileReadError(
                    format!("Error while parsing line {} (Reason: {})", self.lines_read, e.to_string())
                ))),
            }
        }

        if lines.len() == 4 {
            let identifier = lines.remove(0);
            let nucleotides = lines.remove(0);
            let separator = lines.remove(0);
            let quality = lines.remove(0);

            if separator != "+" {
                return Some(Err(
                    FastQError::WrongFormat {
                        malformed_line: separator,
                        mal_line_no: self.lines_read - 1
                    }
                ))
            }

            self.records_read += 1;
            Some(Ok(FastQRecord {
                identifier, nucleotides, quality
            }))
        } else {
            None
        }
    }
}
