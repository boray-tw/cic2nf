#[derive(Copy, Clone, Debug)]
pub struct Flags {
    pub cwr: bool,
    pub ece: bool,
    pub urg: bool,
    pub ack: bool,
    pub psh: bool,
    pub rst: bool,
    pub syn: bool,
    pub fin: bool,
}

impl std::fmt::Display for Flags {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c: &'static str = if self.cwr { "C" } else { "." };
        let e: &'static str = if self.ece { "E" } else { "." };
        let u: &'static str = if self.urg { "U" } else { "." };
        let a: &'static str = if self.ack { "A" } else { "." };
        let p: &'static str = if self.psh { "P" } else { "." };
        let r: &'static str = if self.rst { "R" } else { "." };
        let s: &'static str = if self.syn { "S" } else { "." };
        let f: &'static str = if self.fin { "F" } else { "." };
        write!(formatter, "{}{}{}{}{}{}{}{}", c, e, u, a, p, r, s, f)
    }
}

impl From<[bool; 8]> for Flags {
    fn from(b: [bool; 8]) -> Self {
        Self {
            cwr: b[0],
            ece: b[1],
            urg: b[2],
            ack: b[3],
            psh: b[4],
            rst: b[5],
            syn: b[6],
            fin: b[7],
        }
    }
}

impl From<Vec<bool>> for Flags {
    fn from(flag_list: Vec<bool>) -> Self {
        let mut flag_arr: [bool; 8] = Default::default();
        for (i, f) in flag_list.iter().enumerate() {
            flag_arr[i] = *f;
        }
        flag_arr.into()
    }
}

impl From<&String> for Flags {
    fn from(s: &String) -> Self {
        let flag_list: Vec<bool> = s.chars().map(|f| f != '.').collect();
        flag_list.into()
    }
}

impl From<&str> for Flags {
    fn from(s: &str) -> Self {
        (&s.to_string()).into()
    }
}
