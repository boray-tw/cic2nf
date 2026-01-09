use super::{flag_count::CICFlagCount, label::Label, time::FlowTimeStamp};
use chrono::{Duration, NaiveDateTime};

/*
 * A record (row) in the datasets of the CIC in CSV format.
 * For each array-type field, field[0] is forward and
 * field[1] is backward.
 */
#[derive(Clone, Debug)]
pub struct CICRecord {
    src_ip: String,
    src_port: u32,
    dst_ip: String,
    dst_port: u32,
    protocol: u8,
    timestamp: FlowTimeStamp,
    duration: Duration,
    n_packet: [u32; 2],
    n_bytes_packet: [usize; 2],
    flag_count: CICFlagCount,
    label: Label,
}

impl CICRecord {
    pub fn from_ids_csv(
        record: &csv::StringRecord,
        is_am: &Option<bool>,
        guessed_time_format_index: usize,
        packet_parser_config: &mut CICParserConfig,
    ) -> (CICRecord, usize) {
        let (timestamp, actual_time_format_index) =
            CICRecord::str_to_timestamp(&record[6], is_am, guessed_time_format_index);

        let cic_record: CICRecord = CICRecord {
            src_ip: record[1].into(),
            src_port: record[2].parse().unwrap(),
            dst_ip: record[3].into(),
            dst_port: record[4].parse().unwrap(),
            protocol: record[5].parse().unwrap(),
            timestamp,
            duration: Duration::microseconds(record[7].parse().unwrap()),
            n_packet: packet_parser_config.get_counts(&record[8], &record[9]),
            n_bytes_packet: packet_parser_config.get_bytes(
                &record[40],
                &record[41],
                &record[10],
                &record[11],
            ),
            flag_count: CICFlagCount::new([
                &record[36],
                &record[37],
                &record[38],
                &record[39],
                &record[49],
                &record[50],
                &record[51],
                &record[52],
                &record[53],
                &record[54],
                &record[55],
                &record[56],
            ]),
            label: Label::new_with_name(CICRecord::fix_invalid_utf8_str(&record[84])),
        };

        (cic_record, actual_time_format_index)
    }

    pub fn from_ddos_csv(
        record: &csv::StringRecord,
        is_am: &Option<bool>,
        guessed_time_format_index: usize,
        packet_parser_config: &mut CICParserConfig,
    ) -> (CICRecord, usize) {
        let (timestamp, actual_time_format_index) =
            CICRecord::str_to_timestamp(&record[7], is_am, guessed_time_format_index);

        let cic_record: CICRecord = CICRecord {
            src_ip: record[2].into(),
            src_port: record[3].parse().unwrap(),
            dst_ip: record[4].into(),
            dst_port: record[5].parse().unwrap(),
            protocol: record[6].parse().unwrap(),
            timestamp,
            duration: Duration::microseconds(record[8].parse().unwrap()),
            n_packet: packet_parser_config.get_counts(&record[9], &record[10]),
            n_bytes_packet: packet_parser_config.get_bytes(
                &record[41],
                &record[42],
                &record[11],
                &record[12],
            ),
            flag_count: CICFlagCount::new([
                &record[37],
                &record[38],
                &record[39],
                &record[40],
                &record[50],
                &record[51],
                &record[52],
                &record[53],
                &record[54],
                &record[55],
                &record[56],
                &record[57],
            ]),
            label: Label::new_with_name(CICRecord::fix_invalid_utf8_str(&record[87])),
        };

        (cic_record, actual_time_format_index)
    }

    // ref: https://docs.rs/chrono/0.4.42/chrono/format/strftime/index.html
    const TIME_FORMATS: [&'static str; 5] = [
        "%e/%m/%Y %l:%M %p",     // 3/7/2023 9:59 pm
        "%e/%m/%Y %l:%M:%S %p",  // 3/7/2023 9:59:59 pm
        "%e/%m/%Y %l:%M %P",     // 3/7/2023 9:59 PM
        "%e/%m/%Y %l:%M:%S %P",  // 3/7/2023 9:59:59 PM
        "%Y-%m-%d %H:%M:%S%.6f", // 2023-07-03 21:59:59.123456
    ];

    /**
    Convert string (str) to Timestamp, and along with the
    actual time format index.

    If you deal with multiple records in the same file,
    it is recommended to feed the actual index back as
    the next guessed index.
    */
    fn str_to_timestamp(
        cic_timestamp_str: &str,
        is_am: &Option<bool>,
        guessed_time_format_index: usize,
    ) -> (FlowTimeStamp, usize) {
        let guess_i: usize = guessed_time_format_index;

        let mut time_str: String = cic_timestamp_str.to_owned();
        match is_am {
            Some(true) => time_str += " am",
            Some(false) => time_str += " pm",
            None => {}
        }

        let time_str: &str = time_str.as_str();
        if let Ok(time) = NaiveDateTime::parse_from_str(time_str, Self::TIME_FORMATS[guess_i]) {
            return (FlowTimeStamp::new(time), guess_i);
        }

        for i in 0..Self::TIME_FORMATS.len() {
            if i == guess_i {
                continue;
            }
            if let Ok(time) = NaiveDateTime::parse_from_str(time_str, Self::TIME_FORMATS[i]) {
                return (FlowTimeStamp::new(time), i);
            }
        }

        if is_am.is_none() {
            panic!(
                "Please specify --is-am, --is-pm or --is-am-list for the timestamp: {}",
                time_str
            )
        }

        panic!(
            "Time string is not in the list of known formats:\n  {}\n",
            time_str
        );
    }

    fn fix_invalid_utf8_str(input: &str) -> String {
        if input.contains('\u{FFFD}') {
            input.replace('\u{FFFD}', "-")
        } else {
            String::from(input)
        }
    }

    pub fn id(&self) -> String {
        if self.src_ip == self.dst_ip && self.dst_port < self.src_port {
            format!(
                "{}-{}-{}-{}-{}",
                self.src_ip, self.dst_ip, self.dst_port, self.src_port, self.protocol
            )
        } else if self.src_ip <= self.dst_ip {
            format!(
                "{}-{}-{}-{}-{}",
                self.src_ip, self.dst_ip, self.src_port, self.dst_port, self.protocol
            )
        } else {
            format!(
                "{}-{}-{}-{}-{}",
                self.dst_ip, self.src_ip, self.dst_port, self.src_port, self.protocol
            )
        }
    }

    pub fn src_ip(&self) -> &String {
        &self.src_ip
    }

    pub fn src_port(&self) -> u32 {
        self.src_port
    }

    pub fn dst_ip(&self) -> &String {
        &self.dst_ip
    }

    pub fn dst_port(&self) -> u32 {
        self.dst_port
    }

    pub fn protocol(&self) -> u8 {
        self.protocol
    }

    pub fn timestamp(&self) -> &FlowTimeStamp {
        &self.timestamp
    }

    pub fn duration(&self) -> &Duration {
        &self.duration
    }

    pub fn n_packet(&self) -> &[u32; 2] {
        &self.n_packet
    }

    pub fn n_bytes_packet(&self) -> &[usize; 2] {
        &self.n_bytes_packet
    }

    pub fn flag_count(&self) -> &CICFlagCount {
        &self.flag_count
    }

    pub fn label(&self) -> &Label {
        &self.label
    }

    pub fn label_mut(&mut self) -> &mut Label {
        &mut self.label
    }
}

#[derive(Default)]
pub struct CICParserConfig {
    pub packet_count: PacketPartParser,
    pub packet_bytes: PacketPartParser,
}

impl CICParserConfig {
    fn get_counts(&mut self, fwd: &str, bwd: &str) -> [u32; 2] {
        let output = self.packet_count.parse_usize(fwd, bwd);
        [output[0] as u32, output[1] as u32]
    }

    fn get_bytes(
        &mut self,
        fwd_header: &str,
        bwd_header: &str,
        fwd_payload: &str,
        bwd_payload: &str,
    ) -> [usize; 2] {
        let header: [isize; 2] = self.packet_bytes.parse_isize(fwd_header, bwd_header);
        let payload: [isize; 2] = self.packet_bytes.parse_isize(fwd_payload, bwd_payload);
        let fwd = header[0] + payload[0];
        let bwd = header[1] + payload[1];
        let fwd_out: usize = if fwd < 0 { 0 } else { fwd as usize };
        let bwd_out: usize = if bwd < 0 { 0 } else { bwd as usize };
        [fwd_out, bwd_out]
    }
}

#[derive(Debug, Clone)]
struct CustomParseError {
    was_parsing_float: bool, // false for an integer
}

impl std::fmt::Display for CustomParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let kind = if self.was_parsing_float {
            "a float"
        } else {
            "an integer"
        };

        write!(f, "Cannot parse the string as {}.", kind)
    }
}

/// Parser for a part of a packet.
/// This is the primary building block of `CICParserConfig`.
#[derive(Default)]
pub struct PacketPartParser {
    pub is_last_float: bool,
}

impl PacketPartParser {
    fn parse_usize(&mut self, fwd: &str, bwd: &str) -> [usize; 2] {
        let (fwd_count, is_fwd_float) = self.parse_str_usize(fwd, self.is_last_float);
        let (bwd_count, is_bwd_float) = self.parse_str_usize(bwd, self.is_last_float);
        self.is_last_float = is_fwd_float || is_bwd_float;
        [fwd_count, bwd_count]
    }

    fn parse_isize(&mut self, fwd: &str, bwd: &str) -> [isize; 2] {
        let (fwd_count, is_fwd_float) = self.parse_str_isize(fwd, self.is_last_float);
        let (bwd_count, is_bwd_float) = self.parse_str_isize(bwd, self.is_last_float);
        self.is_last_float = is_fwd_float || is_bwd_float;
        [fwd_count, bwd_count]
    }

    /// Parse an integer or float string, and return usize along with
    /// the flag indicating whether the input string is a float.
    /// ## Arguments
    /// * `s`: An integer or float string.
    /// * `is_from_float`: Is the string `s` expected to be a float?
    fn parse_str_usize(&self, s: &str, is_from_float: bool) -> (usize, bool) {
        if is_from_float {
            return match self.parse_float_str(s) {
                Ok(num) => (num, true),
                Err(_) => self.parse_str_usize(s, false),
            };
        }
        return match s.parse::<usize>() {
            Ok(num) => (num, false),
            Err(_) => (
                self.parse_float_str(s)
                    .expect(format!("Cannot parse {} as an integer or float.", s).as_str()),
                true,
            ),
        };
    }

    fn parse_str_isize(&self, s: &str, is_from_float: bool) -> (isize, bool) {
        let is_negative = s.starts_with("-");

        let input_str = if is_negative { s.get(1..).unwrap() } else { s };

        let sign = if is_negative { -1 } else { 1 };

        let output = self.parse_str_usize(input_str, is_from_float);
        return (sign * (output.0 as isize), output.1);
    }

    fn parse_float_str(&self, s: &str) -> Result<usize, CustomParseError> {
        return match s.find(".") {
            Some(decimal_point_index) => {
                let integer_part: &str = &s[..decimal_point_index];
                parse_str_return_custom_error(integer_part)
            }
            None => Err(CustomParseError {
                was_parsing_float: true,
            }),
        };

        fn parse_str_return_custom_error(s: &str) -> Result<usize, CustomParseError> {
            match s.parse::<usize>() {
                Ok(output_number) => Ok(output_number),
                Err(_) => Err(CustomParseError {
                    was_parsing_float: false,
                }),
            }
        }
    }
}
