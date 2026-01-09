use super::{flag_count::CICFlagCount, time::FlowTimeStamp};
use chrono::Duration;

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
    label: String,
}

impl CICRecord {
    pub fn new(
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
        label: String,
    ) -> Self {
        Self {
            src_ip,
            src_port,
            dst_ip,
            dst_port,
            protocol,
            timestamp,
            duration,
            n_packet,
            n_bytes_packet,
            flag_count,
            label,
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

    pub fn label(&self) -> &String {
        &self.label
    }
}

