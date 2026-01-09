use crate::cic::{parser::CICParser, record::CICRecord};

pub struct CICReader {
    parser: Box<dyn CICParser>,
    file_path: String,
    reader: csv::Reader<std::fs::File>,
}

impl CICReader {
    /// Open a file with a configured parser
    ///
    /// # Errors
    /// This function returns an error if [`csv::ReaderBuilder::from_path()`] fails.
    pub fn open(
        path: &String,
        parser: Box<dyn CICParser>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            parser,
            file_path: path.clone(),
            reader: csv::ReaderBuilder::new()
                .has_headers(true)
                .from_path(path)?,
        })
    }

    pub fn is_done(&self) -> bool {
        self.reader.is_done()
    }

    /// Read at most `n` lines as a vector of [`CICRecord`],
    /// where `n` is default to 1 million.
    ///
    /// # Panics
    /// This function panics if [`csv::Reader::read_byte_record()`] emits an error.
    pub fn read_records(&mut self, n: Option<usize>) -> Vec<CICRecord> {
        let n_lines: usize = n.unwrap_or(1_000_000);
        let mut i_line: usize = 0;
        let mut line_buffer = csv::ByteRecord::new();
        let mut cic_records: Vec<CICRecord> = Vec::with_capacity(n_lines);

        while i_line < n_lines && !self.reader.is_done() {
            // read a line as a byte record
            let has_record = self.reader.read_byte_record(&mut line_buffer).expect(
                format!(
                    "Cannot read file {} since line {}",
                    &self.file_path,
                    self.reader.position().line()
                )
                .as_str(),
            );
            if !has_record {
                break;
            }

            // parse a valid row as a string record
            let string_record;
            match self.parser.fix_byte_record(line_buffer.to_owned()) {
                Some(record) => {
                    string_record = csv::StringRecord::from_byte_record_lossy(record);
                    i_line += 1;
                }
                None => continue,
            }

            // parse the string record as a CIC record
            cic_records.push(
                self.parser.parse_string_record(string_record).expect(
                    format!(
                        "Cannot parse line {}:",
                        self.reader.position().line()
                            + if self.reader.has_headers() { 1 } else { 0 }
                    )
                    .as_str(),
                ),
            )
        }

        return cic_records;
    }
}
