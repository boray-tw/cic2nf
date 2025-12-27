use chrono::{Duration, NaiveDateTime};

use crate::cic::label::Label;

use super::flow::{NetFlow, set_nf_format};
use super::{super::cic::time::FlowTimeStamp, flags::Flags};
use std::fs::OpenOptions;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

struct NetFlowReader;

impl NetFlowReader {
    pub fn new() -> Self {
        Self
    }

    /// 2023-12-31 23:59:59.123
    const TIMESTAMP_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S%.3f";

    pub fn read_line(&self, input_line: &String, label: &Label) -> NetFlow {
        fn next<'a>(iter: &mut dyn Iterator<Item = &'a str>) -> &'a str {
            iter.next().unwrap()
        }

        let mut iter = input_line.split_whitespace().filter(|&s| !s.is_empty());

        let timestamp_string: String = format!("{} {}", next(&mut iter), next(&mut iter));
        let chrono_timestamp: NaiveDateTime =
            NaiveDateTime::parse_from_str(timestamp_string.as_str(), Self::TIMESTAMP_FORMAT)
                .expect(
                    format!(
                        "Unable to parse the timestamp \"{}\" in the format \"{}\"",
                        timestamp_string.as_str(),
                        Self::TIMESTAMP_FORMAT
                    )
                    .as_str(),
                );

        let duration_ms_string: String = next(&mut iter).replace(".", "");
        let duration: Duration = Duration::milliseconds(
            duration_ms_string
                .parse::<i64>()
                .expect(Self::unable_to_parse_str(&duration_ms_string).as_str()),
        );

        let protocol: u8 = next(&mut iter).parse::<u8>().unwrap();

        let (src_ip, src_port) = Self::extract_ip_port(next(&mut iter));
        iter.next(); // skip ->
        let (dst_ip, dst_port) = Self::extract_ip_port(next(&mut iter));

        let flags: Flags = next(&mut iter).into();

        let qos_str: &str = next(&mut iter);
        let qos: u8 = qos_str
            .parse()
            .expect(Self::unable_to_parse_str(qos_str).as_str());

        let n_packet_str = next(&mut iter);
        let n_packet: u32 = n_packet_str
            .parse()
            .expect(Self::unable_to_parse_str(n_packet_str).as_str());

        let n_bytes_packet_str: &str = next(&mut iter);
        let n_bytes_packet: usize = n_bytes_packet_str
            .parse()
            .expect(Self::unable_to_parse_str(n_bytes_packet_str).as_str());

        let n_flow_str: &str = next(&mut iter);
        let n_flow: u32 = n_flow_str
            .parse()
            .expect(Self::unable_to_parse_str(n_flow_str).as_str());

        NetFlow {
            timestamp: FlowTimeStamp::new(chrono_timestamp),
            duration,
            width_duration_str: 0,
            protocol,
            src_ip,
            src_port,
            dst_ip,
            dst_port,
            flags,
            qos,
            n_packet,
            n_bytes_packet,
            width_n_bytes_packet: 0,
            n_flow,
            label: label.clone(),
        }
    }

    fn extract_ip_port(ip_port_pair: &str) -> (String, u32) {
        let components: Vec<&str> = ip_port_pair.split(":").collect();
        (
            components[0].to_string(),
            components[1]
                .parse::<u32>()
                .expect(format!("Cannot parse {} as a number.", components[1]).as_str()),
        )
    }

    fn unable_to_parse_str(s: &str) -> String {
        format!("Unable to parse the string \"{s}\" as a number.")
    }
}

pub fn read_nf_file(fname: &String) -> Vec<NetFlow> {
    let file: File =
        File::open(fname).expect(format!("Cannot read the NetFlow file {}", fname).as_str());
    let no_label = Label::new(0, "NoLabel".to_string());
    let mut flow_storage: Vec<NetFlow> = Vec::new();
    let nf_reader: NetFlowReader = NetFlowReader::new();

    for line_result in BufReader::new(file).lines() {
        let line: String = line_result.expect(format!("Cannot read a line in {}", fname).as_str());
        let flow: NetFlow = nf_reader.read_line(&line, &no_label);
        flow_storage.push(flow);
    }

    set_nf_format(&mut flow_storage, None, None);

    flow_storage
}

pub fn write_nf_file(nf_records: &Vec<NetFlow>, fname: &String) {
    // FIXME: toggle this function with command-line flags
    /*if Path::new(fname).exists() {
        print!("File {} exists. Do you want to overwrite it? [Y/n] ", fname);
        let mut buffer = String::new();
        stdin()
            .read_line(&mut buffer)
            .expect("Error: Cannot read from stdin.");
        if buffer == "n" {
            println!("Skipped file: {}", fname);
            return;
        }
    }*/
    let of = OpenOptions::new()
        .append(true)
        .create(true)
        .open(fname.to_string())
        .expect(&format!("Unable to create/edit file {}", fname).to_string());

    let mut ob = BufWriter::new(of);

    for line in nf_records {
        writeln!(ob, "{}", line).expect(&format!(
            "Unable to write the following content to file {}\n{}",
            fname, line
        ));
    }
}
