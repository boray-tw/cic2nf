use std::collections::HashMap;

use cic2nf::{
    cic::{
        dataset_name::DatasetName,
        io::CICReader,
        parser::{CICDdos2019Parser, CICIds2017Parser, CICParser},
        record::CICRecord,
    },
    nf::{
        flow::{NetFlow, categorize_nf, cic_to_nf_batch},
        io::write_nf_file,
    },
};
use clap::{ArgAction, Command, arg, crate_version};

const PROGRAM_NAME: &'static str = "csv_to_nf";

const SHORT_DESCRIPTION: &'static str = "\
    Convert a CIC dataset in CSV format \
    to NetFlow v5 files categorized by labels. \
";

pub fn get_cli_parser() -> Command {
    Command::new(PROGRAM_NAME)
        .about(SHORT_DESCRIPTION)
        .next_line_help(true)
        .arg_required_else_help(true)
        .version(crate_version!())
        .arg(
            arg!(-n --name <NAME> "The name of the input dataset")
                .required(true)
                .value_parser(["CIC-IDS-2017", "CIC-DDoS-2019"]),
        )
        .arg(
            arg!(-o --output_dir <OUT_DIR> "The output directory.")
                .required(false)
                .default_value("./output/netflow-categorized/"),
        )
        .arg(
            arg!(input_paths: <IN_PATH> "The path(s) to the input file(s)")
                .required(true)
                .action(ArgAction::Append),
        )
}

pub fn convert_cic_file_to_nf_files(
    dataset_name: &DatasetName,
    in_path: &String,
    out_dir: &String,
) {
    // initialization
    let parser: Box<dyn CICParser> = match dataset_name {
        DatasetName::IDS2017 => Box::new(CICIds2017Parser::new(in_path)),
        DatasetName::DDoS2019 => Box::new(CICDdos2019Parser::new()),
    };
    let mut reader: CICReader =
        CICReader::open(in_path, parser).expect(format!("Cannot open a file {in_path}").as_str());

    std::fs::create_dir_all(out_dir)
        .expect(&format!("Unable to create output directory {}", out_dir).to_string());

    while !reader.is_done() {
        // read a batch
        let cic_records: Vec<CICRecord> = reader.read_records(None);

        // convert to NetFlow
        let nf_records: Vec<NetFlow> = cic_to_nf_batch(cic_records)
            .expect(format!("Unable to convert CICRecord's in {} to NetFlow's.", out_dir).as_str());

        // categorize flows by labels
        let categorized_nf_records: HashMap<String, Vec<NetFlow>> = categorize_nf(nf_records);

        // store categorized flows
        for (label_name, nf_records) in categorized_nf_records {
            write_nf_file(&nf_records, &format!("{out_dir}/{label_name}.nf"));
        }
    }
}
