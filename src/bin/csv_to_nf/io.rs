use cic2nf::{
    cic::{dataset_name::DatasetName, io::read_csv},
    nf::{
        flow::{NetFlow, categorize_nf, cic_to_nf_batch},
        io::write_nf_file,
    },
};
use clap::{Arg, ArgAction, Command, arg, crate_version};
use log::warn;

const PROGRAM_NAME: &'static str = "csv_to_nf";

const SHORT_DESCRIPTION: &'static str = "\
    Convert a CIC dataset in CSV format \
    to NetFlow v5 files categorized by labels. \
";

const IS_AM_HELP_MESSAGE: &'static str = "\
    For all input files, the timestamp is in 12-hour format, \
    and does not come with AM or PM suffix, \
    but it supposes to be in the morning. \
    (Relevant datasets: CIC-IDS-2017.)
    (Irrelevant datasets: CIC-DDoS-2019.)
";

const IS_AM_LIST_HELP_MESSAGE: &'static str = "\
    Similar to --is_am, but it is a comma-separated list consisting of \
    elements in the following meanings for input files in order:
    0 or n: It is in 24-hour clock, or with AM or PM suffix.
    1 or a: --is_am for this file.
    2 or p: --is_pm for this file.
";

pub fn get_cli_parser() -> Command {
    Command::new(PROGRAM_NAME)
        .about(SHORT_DESCRIPTION)
        .next_line_help(true)
        .arg_required_else_help(true)
        .version(crate_version!())
        .arg(
            Arg::new("is_am")
                .long("is_am")
                .help(IS_AM_HELP_MESSAGE)
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("is_pm")
                .long("is_pm")
                .help("Similar to --is_am, but now it supposes to be in the evening")
                .required(false)
                .action(ArgAction::SetTrue)
                .conflicts_with("is_am"),
        )
        .arg(
            Arg::new("is_am_list")
                .long("is_am_list")
                .help(IS_AM_LIST_HELP_MESSAGE)
                .value_delimiter(',')
                .conflicts_with("is_am"),
        )
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

pub fn parse_and_fix_is_am_list(is_am_list: Vec<&String>, n_input: usize) -> Vec<Option<bool>> {
    fn parse_flag(flag_string: &&String) -> Option<bool> {
        let flag: &str = flag_string.as_str();
        match flag {
            "0" | "n" => None,
            "1" | "a" => Some(true),
            "2" | "p" => Some(false),
            _ => panic!("Invalid is_am_list entry: {}", flag),
        }
    }

    fn parse_list(fixed_list: Vec<&String>) -> Vec<Option<bool>> {
        fixed_list.iter().map(parse_flag).collect()
    }

    if is_am_list.len() == n_input {
        return parse_list(is_am_list);
    }

    if is_am_list.len() > n_input {
        warn!(
            "Excess {} elements in is_am_list is truncated.",
            is_am_list.len() - n_input
        );
        return parse_list(is_am_list[..n_input].to_vec());
    }

    let n_missing: usize = n_input - is_am_list.len();
    warn!(
        "is_am_list lacks {} elements, filled with the last one.",
        n_missing
    );
    let first_part = parse_list(is_am_list);
    let last_part = vec![*first_part.last().unwrap(); n_missing];
    return itertools::concat([first_part, last_part]);
}

pub fn convert_cic_file_to_nf_files(
    dataset_name: &DatasetName,
    in_path: &String,
    out_dir: &String,
    is_am: &Option<bool>,
) {
    let (cic_records, label_library) = read_csv(dataset_name, in_path, is_am)
        .expect(&format!("Unable to load {}", in_path).as_str());

    let nf_records: Vec<NetFlow> = cic_to_nf_batch(cic_records)
        .expect(&format!("Unable to convert CICRecord's in {} to NetFlow's.", out_dir).to_string());

    std::fs::create_dir_all(out_dir)
        .expect(&format!("Unable to create output directory {}", out_dir).to_string());

    let categorized_nf_records: Vec<Vec<NetFlow>> = categorize_nf(nf_records, label_library);

    for nf_one_category in categorized_nf_records {
        if nf_one_category.is_empty() {
            continue;
        }
        let label_name = nf_one_category[0].label().name();
        let out_path: String = format!("{}/{}.nf", out_dir, label_name);
        write_nf_file(&nf_one_category, &out_path);
    }
}
