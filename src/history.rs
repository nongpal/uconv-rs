use serde::{Deserialize, Serialize};
use std::fs;
use std::io::ErrorKind;

#[derive(Serialize, Deserialize, Debug)]
pub struct History<T> {
    from_unit: T,
    to_unit: T,
    value_unit: f64,
    output_unit: f64,
}

const FILE_NAME: &str = "conversion.json";

pub fn load_history() -> Vec<History> {
    let data = match fs::read_to_string(FILE_NAME) {
        Ok(d) => d,
        Err(_) => return vec![],
    };

    serde_json::from_str(&data).unwrap_or_else(|_| vec![])
}

pub fn save_history(history: &Vec<History>) {
    if let Ok(json) = serde_json::to_string_pretty(history) {
        let _ = fs::write(FILE_NAME, json);
    }
}
