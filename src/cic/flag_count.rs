use crate::nf::flags::Flags;

#[derive(Clone, Debug)]
pub struct CICFlagCount {
    fwd_psh: u32,
    bwd_psh: u32,
    fwd_urg: u32,
    bwd_urg: u32,
    fin: u32,
    syn: u32,
    rst: u32,
    psh: u32,
    ack: u32,
    urg: u32,
    cwr: u32,
    ece: u32,
}

impl CICFlagCount {
    /**
    Construct flags from ordered `&str`s.

    The order is as follows, 0-indexed.

    *  0: Fwd PSH Flags
    *  1: Bwd PSH Flags
    *  2: Fwd URG Flags
    *  3: Bwd URG Flags
    *  4: FIN Flag Count
    *  5: SYN Flag Count
    *  6: RST Flag Count
    *  7: PSH Flag Count
    *  8: ACK Flag Count
    *  9: URG Flag Count
    * 10: CWR Flag Count
    * 11: ECE Flag Count
    */
    pub fn new(s: [&str; 12]) -> Self {
        CICFlagCount {
            fwd_psh: s[0].parse::<u32>().unwrap(),
            bwd_psh: s[1].parse::<u32>().unwrap(),
            fwd_urg: s[2].parse::<u32>().unwrap(),
            bwd_urg: s[3].parse::<u32>().unwrap(),
            fin: s[4].parse::<u32>().unwrap(),
            syn: s[5].parse::<u32>().unwrap(),
            rst: s[6].parse::<u32>().unwrap(),
            psh: s[7].parse::<u32>().unwrap(),
            ack: s[8].parse::<u32>().unwrap(),
            urg: s[9].parse::<u32>().unwrap(),
            cwr: s[10].parse::<u32>().unwrap(),
            ece: s[11].parse::<u32>().unwrap(),
        }
    }
}

impl Into<Flags> for &CICFlagCount {
    fn into(self) -> Flags {
        Flags {
            psh: (self.fwd_psh != 0) || (self.bwd_psh != 0) || (self.psh != 0),
            urg: (self.fwd_urg != 0) || (self.bwd_urg != 0) || (self.urg != 0),
            fin: self.fin != 0,
            syn: self.syn != 0,
            rst: self.rst != 0,
            ack: self.ack != 0,
            cwr: self.cwr != 0,
            ece: self.ece != 0,
        }
    }
}
