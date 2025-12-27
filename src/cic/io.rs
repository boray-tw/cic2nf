use super::record::CICParserConfig;
use super::{dataset_name::DatasetName, record::CICRecord};
use csv::{Reader, ReaderBuilder};
use log::warn;
use std::collections::HashMap;
use std::fs::File;

/**
 Read CSV data stored in `path` of dataset name `dataset_name`,
 and return a vector of CICRecord and a HashMap
 consisted of available `Label`s.

Parameters:
*  `dataset_name`: DatasetName. Which dataset do you
want to read?

* `path`: &String. Relative or absolute path to the CSV file to read.
path separators can be either `/` or `\`.
For example, `input/cic_ids_ddos.csv` or `D:\datasets\CIC\IDS-2017\data.csv`.

* `is_am`: &Option<bool>. If the timestamp of the input CSV is in 24-hour clock,
or it is in 12-hour clock along with AM (am) or PM (pm),
set this parameter to `None`. Otherwise, set this parameter to `Some(true)`
if the timestamp is implicitly in AM, or `Some(false)` for PM.
 */
pub fn read_csv(
    dataset_name: &DatasetName,
    path: &String,
    is_am: &Option<bool>,
) -> std::io::Result<(Vec<CICRecord>, HashMap<String, u8>)> {
    let benign_label: (String, u8);
    let n_csv_column: usize;
    let cic_record_initializer: fn(
        &csv::StringRecord,
        &Option<bool>,
        usize,
        &mut CICParserConfig,
    ) -> (CICRecord, usize);

    match dataset_name {
        DatasetName::IDS2017 => {
            benign_label = ("BENIGN".to_string(), 1);
            n_csv_column = 85;
            cic_record_initializer = CICRecord::from_ids_csv;
        }
        DatasetName::DDoS2019 => {
            benign_label = ("BENIGN".to_string(), 1);
            n_csv_column = 88;
            cic_record_initializer = CICRecord::from_ddos_csv;
        }
    }

    let mut label_map: HashMap<String, u8> = HashMap::from([benign_label]);
    let mut cic_record_storage: Vec<CICRecord> = Vec::new();
    let mut cic_record: CICRecord;
    let mut time_format_index: usize = 0;
    let mut packet_parser_config: CICParserConfig = Default::default();

    let mut csv_reader: Reader<File> = ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)
        .expect(&format!("Unable to read CSV file: {}", path).as_str());

    for byte_record in csv_reader.byte_records() {
        // replace invalid UTF-8 characters with U+FFFD
        let str_record: csv::StringRecord = csv::StringRecord::from_byte_record_lossy(byte_record?);

        // skip mismatched dataset dimensions
        if str_record.len() != n_csv_column {
            warn!(
                "Skipped CSV record (expected # of col {} but get {}): {:?}",
                n_csv_column,
                str_record.len(),
                str_record
            );
            continue;
        }

        // skip failure row
        if str_record.iter().all(|s| s.is_empty()) {
            continue;
        }

        (cic_record, time_format_index) = cic_record_initializer(
            &str_record,
            is_am,
            time_format_index,
            &mut packet_parser_config,
        );

        update_label_and_index_mut(&mut label_map, &mut cic_record);
        cic_record_storage.push(cic_record);
    }

    Ok((cic_record_storage, label_map))
}

fn update_label_and_index_mut(label_map: &mut HashMap<String, u8>, cic_record: &mut CICRecord) {
    let current_label: &String = &cic_record.label().name();
    match label_map.get(current_label) {
        Some(index) => cic_record.label_mut().set_index_mut(*index),
        None => {
            let current_index: u8 = (label_map.len() + 1) as u8;
            label_map.insert(current_label.clone(), current_index);
            cic_record.label_mut().set_index_mut(current_index);
        }
    }
}
