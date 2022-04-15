use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub units: Units,
    pub location: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Units {
    Metric,
    Imperial
}

impl Default for Units {
    fn default() -> Self {
        Units::Metric
    }
}
