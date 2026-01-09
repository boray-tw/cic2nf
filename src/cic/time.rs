use chrono::NaiveDateTime;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FlowTimeStamp {
    time: NaiveDateTime,
}

impl FlowTimeStamp {
    pub fn new(time: NaiveDateTime) -> Self {
        Self { time }
    }
}

// format as a NetFlow timestamp
impl std::fmt::Display for FlowTimeStamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.time.format("%Y-%m-%d %H:%M:%S%.3f"))
    }
}
