use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub filter_label: String,
    pub entries_label: String,
}
