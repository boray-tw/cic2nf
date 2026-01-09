use chrono::{Duration, NaiveDateTime, Timelike};
use csv::{ByteRecord, StringRecord};
use log::warn;
use std::{num::ParseIntError, path::Path, str::FromStr};

use crate::cic::{flag_count::CICFlagCount, record::CICRecord, time::FlowTimeStamp};

pub trait CICParser {
    /// Fix the data of a row of CIC's CSV file in byte level.
    ///
    /// Return fixed valid data, or return `None` for invalid data to discard.
    fn fix_byte_record(&self, record: ByteRecord) -> Option<ByteRecord> {
        Some(record)
    }

    fn parse_string_record(&mut self, record: StringRecord) -> Result<CICRecord, ParseIntError>;
}

enum FuzzyTimePeriod {
    /// hours of 01-11 (01:00 - 11:59)
    Morning,
    /// hours of 11, 12, 01 (11:00 - 13:59)
    Noon,
    /// hours of 01-11 (13:00 - 23:59)
    Afternoon,
    /// hours of 11, 12, 01 (23:00 - 01:59)
    Midnight,
}

struct TimestampParser {
    is_twelve_hour_clock: bool,
    time_period: FuzzyTimePeriod,
    /// References:
    /// * Rust time pattern: [`chrono::format::strftime`]
    /// * pattern used in CICFlowMeter: https://github.com/ahlashkari/CICFlowMeter/blob/98a5eba/src/main/java/cic/cs/unb/ca/jnetpcap/BasicFlow.java#L600
    /// * time patterns in Java: https://docs.oracle.com/javase/8/docs/api/java/time/format/DateTimeFormatter.html
    /// * 12-hour time construction in Java: https://stackoverflow.com/questions/6907968#comment94187480_16190056
    timestamp_pattern: String,
}

impl TimestampParser {
    const TWELVE_HOURS: Duration = Duration::hours(12);

    fn parse(&mut self, timestamp: &str) -> FlowTimeStamp {
        // parse it as a 24-hour time
        let mut time = NaiveDateTime::parse_from_str(timestamp, &self.timestamp_pattern).expect(
            format!(
                "Cannot parse timestamp \"{timestamp}\" using pattern \"{}\".",
                self.timestamp_pattern
            )
            .as_str(),
        );

        if !self.is_twelve_hour_clock {
            return FlowTimeStamp::new(time);
        }

        // handle 12-hour time
        // note: 12 AM = 0:00, 12 PM = 12:00, no 0 AM/PM
        match self.time_period {
            FuzzyTimePeriod::Morning => {
                if time.hour() == 12 {
                    self.time_period = FuzzyTimePeriod::Noon;
                }
            }
            FuzzyTimePeriod::Noon => {
                if time.hour() < 11 {
                    time += Self::TWELVE_HOURS;
                }
                if time.hour() == 2 {
                    self.time_period = FuzzyTimePeriod::Afternoon;
                }
            }
            FuzzyTimePeriod::Afternoon => {
                if time.hour() == 12 {
                    time -= Self::TWELVE_HOURS;
                    self.time_period = FuzzyTimePeriod::Midnight;
                } else {
                    time += Self::TWELVE_HOURS;
                }
            }
            FuzzyTimePeriod::Midnight => match time.hour() {
                11 => time += Self::TWELVE_HOURS,
                12 => time -= Self::TWELVE_HOURS,
                2 => self.time_period = FuzzyTimePeriod::Morning,
                _ => (),
            },
        }

        FlowTimeStamp::new(time)
    }
}

fn parse_counts(fwd: &str, bwd: &str) -> Result<[u32; 2], std::num::ParseIntError> {
    Ok([parse_integer::<u32>(fwd)?, parse_integer::<u32>(bwd)?])
}

fn parse_bytes(
    fwd_payload: &str,
    bwd_payload: &str,
    fwd_header: &str,
    bwd_header: &str,
) -> Result<[usize; 2], std::num::ParseIntError> {
    let fwd: isize = parse_integer::<isize>(fwd_payload)? + parse_integer::<isize>(fwd_header)?;
    let bwd: isize = parse_integer::<isize>(bwd_payload)? + parse_integer::<isize>(bwd_header)?;
    Ok([
        if fwd >= 0 { fwd as usize } else { 0 },
        if bwd >= 0 { fwd as usize } else { 0 },
    ])
}

/// Parse a string as integer but drop the decimal points,
/// and regard a negative number as zero.
fn parse_integer<T: FromStr>(s: &str) -> Result<T, <T as FromStr>::Err> {
    match s.find(".") {
        Some(decimal_point_index) => s[..decimal_point_index].parse::<T>(),
        None => s.parse::<T>(),
    }
}

pub struct CICIds2017Parser {
    timestamp_parser: TimestampParser,
}

impl CICIds2017Parser {
    const N_CSV_COLUMNS: usize = 85;

    pub fn new(input_path: &String) -> Self {
        let filename: String = Path::new(input_path)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into();
        Self {
            timestamp_parser: TimestampParser {
                is_twelve_hour_clock: true,
                time_period: Self::infer_time_period(&filename),
                timestamp_pattern: Self::infer_timestamp_pattern(&filename),
            },
        }
    }

    fn infer_time_period(filename: &String) -> FuzzyTimePeriod {
        if filename.contains("Morning")
            || ["Morning", "Monday", "Tuesday", "Wednesday"]
                .iter()
                .any(|&s| filename.starts_with(s))
        {
            FuzzyTimePeriod::Morning
        } else if filename.contains("Afternoon") {
            FuzzyTimePeriod::Afternoon
        } else {
            warn!("The name of the input file is not standardized: {filename}");
            FuzzyTimePeriod::Morning
        }
    }

    fn infer_timestamp_pattern(filename: &String) -> String {
        if filename.starts_with("Monday") {
            "%d/%m/%Y %H:%M:%S"
        } else {
            "%e/%m/%Y %k:%M"
        }
        .to_string()
    }
}

impl CICParser for CICIds2017Parser {
    fn fix_byte_record(&self, record: ByteRecord) -> Option<ByteRecord> {
        // validate dataset dimensions
        if record.len() != Self::N_CSV_COLUMNS {
            warn!(
                "Skipped CSV record (expected # of col {} but get {}): {:?}",
                Self::N_CSV_COLUMNS,
                record.len(),
                record
            );
            return None;
        }

        // skip empty records, which appear in the last ~289k lines
        // of the Thursday morning data
        if record.iter().all(|s| s.is_empty()) {
            return None;
        }

        // if no "dash" fix is needed, return the original record
        const WINDOWS_1252_DASH: u8 = 0x96;
        const ASCII_DASH: u8 = 0x2d;
        let i_last = record.len() - 1;
        if !record[i_last].contains(&WINDOWS_1252_DASH) {
            return Some(record);
        }

        // reconstruct the record without the last field (label)
        let mut fixed_record = ByteRecord::with_capacity(record.as_slice().len(), record.len());
        for field in record.iter().take(i_last) {
            fixed_record.push_field(field);
        }

        // fix the last field by replacing all Windows dashes with ASCII ones
        let fixed_label: Vec<u8> = record[i_last]
            .iter()
            .map(|&b| {
                if b == WINDOWS_1252_DASH {
                    ASCII_DASH
                } else {
                    b
                }
            })
            .collect();

        // return the fixed record
        fixed_record.push_field(&fixed_label);
        Some(fixed_record)
    }

    /// Parse a [`StringRecord`] as [`CICRecord`], and update
    /// the timestamp parsing rules if needed
    fn parse_string_record(&mut self, record: StringRecord) -> Result<CICRecord, ParseIntError> {
        Ok(CICRecord::new(
            record[1].into(),
            record[2].parse()?,
            record[3].into(),
            record[4].parse()?,
            record[5].parse()?,
            self.timestamp_parser.parse(&record[6]),
            chrono::Duration::microseconds(record[7].parse()?),
            parse_counts(&record[8], &record[9])?,
            parse_bytes(&record[10], &record[11], &record[40], &record[41])?,
            CICFlagCount::new([
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
            record[84].to_string(),
        ))
    }
}

pub struct CICDdos2019Parser {
    timestamp_parser: TimestampParser,
}

impl CICDdos2019Parser {
    const N_CSV_COLUMNS: usize = 88;

    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl CICParser for CICDdos2019Parser {
    fn fix_byte_record(&self, record: ByteRecord) -> Option<ByteRecord> {
        // validate dataset dimensions
        if record.len() != Self::N_CSV_COLUMNS {
            warn!(
                "Skipped CSV record (expected # of col {} but get {}): {:?}",
                Self::N_CSV_COLUMNS,
                record.len(),
                record
            );
            return None;
        }

        // return a valid record
        Some(record)
    }

    /// Parse a [`StringRecord`] as [`CICRecord`], and update
    /// the timestamp parsing rules if needed
    fn parse_string_record(&mut self, record: StringRecord) -> Result<CICRecord, ParseIntError> {
        Ok(CICRecord::new(
            record[2].into(),
            record[3].parse()?,
            record[4].into(),
            record[5].parse()?,
            record[6].parse()?,
            self.timestamp_parser.parse(&record[7]),
            chrono::Duration::microseconds(record[8].parse()?),
            parse_counts(&record[9], &record[10])?,
            parse_bytes(&record[11], &record[12], &record[41], &record[42])?,
            CICFlagCount::new([
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
            record[87].to_string(),
        ))
    }
}

impl Default for CICDdos2019Parser {
    fn default() -> Self {
        Self {
            timestamp_parser: TimestampParser {
                is_twelve_hour_clock: false,
                time_period: FuzzyTimePeriod::Morning, // don't care
                timestamp_pattern: "%Y-%m-%d %H:%M:%S%.6f".to_string(),
            },
        }
    }
}
