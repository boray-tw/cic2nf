use super::flags::Flags;
use crate::cic::{record::CICRecord, time::FlowTimeStamp};
use chrono::Duration;
use log::warn;
use std::{cmp::max, collections::HashMap};

#[derive(Clone, Debug)]
pub struct NetFlow {
    pub timestamp: FlowTimeStamp,
    pub duration: Duration,
    pub width_duration_str: u8,
    pub protocol: u8,
    pub src_ip: String,
    pub src_port: u32,
    pub dst_ip: String,
    pub dst_port: u32,
    pub flags: Flags,
    pub qos: u8,
    pub n_packet: u32,
    pub n_bytes_packet: usize,
    pub width_n_bytes_packet: u8,
    pub n_flow: u32,
    pub label: String,
}

const DURATION_ZERO: Duration = Duration::zero();

impl From<CICRecord> for [Option<NetFlow>; 2] {
    fn from(cr: CICRecord) -> Self {
        let mut duration: Duration = *cr.duration();
        if duration < DURATION_ZERO {
            warn!("Negative duration is converted to 0; ID = {}", cr.id());
            duration = DURATION_ZERO;
        }

        let timestamp: FlowTimeStamp = *cr.timestamp();
        let protocol: u8 = cr.protocol();
        let src_ip: &String = cr.src_ip();
        let src_port: u32 = cr.src_port();
        let dst_ip: &String = cr.dst_ip();
        let dst_port: u32 = cr.dst_port();
        let n_packet: &[u32; 2] = cr.n_packet();
        let n_bytes_packet: &[usize; 2] = cr.n_bytes_packet();
        let flags: Flags = cr.flag_count().into();
        let label: &String = cr.label();

        // construct NetFlow object for each direction,
        // if there is any packet in that direction
        let nf1: Option<NetFlow> = if n_packet[0] == 0 {
            None
        } else {
            Some(NetFlow {
                timestamp,
                duration,
                width_duration_str: 0,
                protocol,
                src_ip: src_ip.clone(),
                src_port,
                dst_ip: dst_ip.clone(),
                dst_port,
                flags: flags.clone(),
                qos: 0,
                n_packet: n_packet[0],
                n_bytes_packet: n_bytes_packet[0],
                width_n_bytes_packet: 0,
                n_flow: 1,
                label: label.clone(),
            })
        };

        let nf2: Option<NetFlow> = if n_packet[1] == 0 {
            None
        } else {
            Some(NetFlow {
                timestamp,
                duration,
                width_duration_str: 0,
                protocol,
                src_ip: dst_ip.clone(),
                src_port: dst_port,
                dst_ip: src_ip.clone(),
                dst_port: src_port,
                flags: flags,
                qos: 0,
                n_packet: n_packet[1],
                n_bytes_packet: n_bytes_packet[1],
                width_n_bytes_packet: 0,
                n_flow: 1,
                label: label.clone(),
            })
        };

        [nf1, nf2]
    }
}

impl NetFlow {
    pub fn label(&self) -> &String {
        &self.label
    }

    fn duration_ms(&self) -> i64 {
        self.duration.num_milliseconds()
    }

    fn format_duration(&self) -> String {
        let ms: i64 = self.duration_ms();
        let s: i64 = ms / 1000;
        let ms: i64 = ms % 1000;
        let tmp_output: String = format!("{}.{:0w$}", s, ms, w = 3);
        format!("{tmp_output:>w$}", w = self.width_duration_str as usize)
    }

    fn format_n_bytes_packet(&self) -> String {
        format!(
            "{:>w$}",
            self.n_bytes_packet,
            w = self.width_n_bytes_packet as usize
        )
    }
}

impl std::fmt::Display for NetFlow {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{} {} {:>3} {:>15}:{:<5} ->   {:>15}:{:<5} {:<8} {:>3} {:>8} {:>8} {:>5}",
            self.timestamp,
            self.format_duration(),
            self.protocol,
            self.src_ip,
            self.src_port,
            self.dst_ip,
            self.dst_port,
            self.flags,
            self.qos,
            self.n_packet,
            self.format_n_bytes_packet(),
            self.n_flow
        )
    }
}

pub fn set_nf_format(
    nf_records: &mut Vec<NetFlow>,
    max_duration_ms_in: Option<i64>,
    max_packet_bytes_in: Option<usize>,
) {
    let max_duration_ms: i64 = if let Some(ms) = max_duration_ms_in {
        ms
    } else {
        let max_record = nf_records
            .iter()
            .max_by(|a, b| a.duration.partial_cmp(&b.duration).unwrap());
        match max_record {
            Some(max) => max.duration_ms(),
            None => 0,
        }
    };

    let max_packet_bytes: usize = if let Some(mb) = max_packet_bytes_in {
        mb
    } else {
        let max_record = nf_records
            .iter()
            .max_by(|a, b| a.n_bytes_packet.partial_cmp(&b.n_bytes_packet).unwrap());
        match max_record {
            Some(max) => max.n_bytes_packet,
            None => 0,
        }
    };

    // get minimal duration column width that makes duration not overflow
    let mut duration_width = get_n_digit_in_decimal(max_duration_ms) + 1;
    if max_duration_ms < 1000 {
        duration_width += 1;
    };

    // get minimal n_byte_packet column width that makes n_byte_packet not overflow
    let n_byte_packet_width = get_n_digit_in_decimal(max_packet_bytes as i64);

    for n in nf_records.iter_mut() {
        n.width_duration_str = duration_width;
        n.width_n_bytes_packet = n_byte_packet_width;
    }

    // sort flows with non-decreasing timestamps, and
    // follow the input orders for the flows with the same timestamp
    nf_records.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
}

pub fn cic_to_nf_batch(cic_records: Vec<CICRecord>) -> std::io::Result<Vec<NetFlow>> {
    let mut netflow_storage: Vec<NetFlow> = Vec::new();
    let mut max_duration_ms = 0;
    let mut max_n_bytes_packet = 0;
    for r in cic_records {
        let nf_arr: [Option<NetFlow>; 2] = r.into();
        for optional_nf in nf_arr {
            if let Some(nf) = optional_nf {
                max_duration_ms = max(max_duration_ms, nf.duration_ms());
                max_n_bytes_packet = max(max_n_bytes_packet, nf.n_bytes_packet);
                netflow_storage.push(nf);
            }
        }
    }

    set_nf_format(
        &mut netflow_storage,
        Some(max_duration_ms),
        Some(max_n_bytes_packet),
    );

    Ok(netflow_storage)
}

fn get_n_digit_in_decimal(mut x: i64) -> u8 {
    if x == 0 {
        return 1;
    }

    let mut n: u8 = 0;
    while x != 0 {
        x /= 10;
        n += 1;
    }

    n
}

/// Categorize NetFlow records by their labels, and return a dictionary
/// with label name as the keys, corresponding records as the values.
///
/// # Arguments
/// * `nf_records` - A vector of NetFlow
pub fn categorize_nf<I>(nf_records: I) -> HashMap<String, Vec<NetFlow>>
where
    I: IntoIterator<Item = NetFlow>,
{
    let mut categorized_records: HashMap<String, Vec<NetFlow>> = HashMap::new();

    for nf in nf_records {
        categorized_records
            .entry(nf.label().to_owned())
            .or_insert(vec![])
            .push(nf);
    }

    return categorized_records;
}
