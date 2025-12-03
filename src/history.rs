use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct History {
    pub from_unit: String,
    pub to_unit: String,
    pub value_unit: f64,
    pub output_unit: f64,
}

const FILE_NAME: &str = "conversion.json";

pub fn read_history() -> Vec<History> {
    if let Ok(data) = fs::read_to_string(FILE_NAME) {
        if let Ok(history) = serde_json::from_str::<Vec<History>>(&data) {
            return history;
        }
    }

    Vec::new()
}

pub fn save_history(entry: History) {
    let mut history: Vec<History> = read_history();

    history.push(entry);

    let json = serde_json::to_string_pretty(&history).expect("Failed to serialize history T_T.");

    let mut file = fs::File::create(FILE_NAME).expect("Failed to create history file T_T");

    file.write_all(json.as_bytes())
        .expect("Failed to write history");
}

pub fn run_history() {
    let hist = read_history();

    if hist.is_empty() {
        println!("No histroy found. Try to convert first.");
        println!("Hint: run `unitconv convert --help`");
        return;
    }

    println!("Conversion History:");
    for (i, h) in hist.iter().enumerate() {
        println!(
            "{}. {} {} = {} {}",
            i + 1,
            h.value_unit,
            h.from_unit,
            h.output_unit,
            h.to_unit,
        );
    }
}
