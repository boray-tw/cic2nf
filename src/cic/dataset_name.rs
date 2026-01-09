pub enum DatasetName {
    IDS2017,
    DDoS2019,
}

impl DatasetName {
    pub fn get_all_enum() -> Vec<DatasetName> {
        vec![Self::IDS2017, Self::DDoS2019]
    }
}

// Note: please change the arguments of value_parser() in
// "src/bin/cic_to_nf/main.rs" to match the string values.
impl std::fmt::Display for DatasetName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatasetName::IDS2017 => f.write_str("CIC-IDS-2017"),
            DatasetName::DDoS2019 => f.write_str("CIC-DDoS-2019"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseDatasetNameError;

impl std::str::FromStr for DatasetName {
    type Err = ParseDatasetNameError;

    fn from_str(input_str: &str) -> Result<Self, Self::Err> {
        let input_string: String = String::from(input_str);
        let dataset_name_pool: Vec<DatasetName> = DatasetName::get_all_enum();
        for name in dataset_name_pool {
            if input_string == name.to_string() {
                return Ok(name);
            }
        }

        return Err(ParseDatasetNameError);
    }
}
