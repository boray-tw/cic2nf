mod io;

use cic2nf::cic::dataset_name::DatasetName;
use clap::ArgMatches;
use io::{convert_cic_file_to_nf_files, get_cli_parser, parse_and_fix_is_am_list};
use log::info;
use std::str::FromStr;

fn main() {
    env_logger::init();

    // load command-line options
    let matches: ArgMatches = get_cli_parser().get_matches();

    let input_paths: Vec<&String> = matches.get_many::<String>("input_paths").unwrap().collect();

    let is_am: Option<bool> = if matches.get_flag("is_am") {
        Some(true)
    } else if matches.get_flag("is_pm") {
        Some(false)
    } else {
        None
    };

    let n_input: usize = input_paths.len();
    let is_am_list: Vec<Option<bool>> = match matches.get_many::<String>("is_am_list") {
        Some(values) => parse_and_fix_is_am_list(values.collect(), n_input),
        None => vec![is_am; n_input],
    };

    let dataset_name_str: String = matches.get_one::<String>("name").unwrap().clone();
    let dataset_name: DatasetName = DatasetName::from_str(dataset_name_str.as_str())
        .expect(format!("Unavailable dataset name: {}", dataset_name_str).as_str());

    let output_dir: &String = matches.get_one::<String>("output_dir").unwrap();

    for (input_path, is_am) in std::iter::zip(input_paths, is_am_list) {
        info!("Processing: {input_path}");
        convert_cic_file_to_nf_files(&dataset_name, input_path, output_dir, &is_am);
    }
    info!("Done.")
}
