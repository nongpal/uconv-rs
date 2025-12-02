mod error;
pub mod length;
pub mod temperature;
pub mod weight;

use std::str::FromStr;

use length::LengthUnit;
use temperature::TemperatureUnit;
use weight::WeightUnit;

use self::error::{UnitError, handle_error};

enum MetricSystem {
    Tempr(TemperatureUnit),
    Len(LengthUnit),
    Weight(WeightUnit),
}

impl FromStr for MetricSystem {
    type Err = UnitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(t) = TemperatureUnit::from_str(&s) {
            return Ok(MetricSystem::Tempr(t));
        }

        if let Ok(l) = LengthUnit::from_str(&s) {
            return Ok(MetricSystem::Len(l));
        }

        if let Ok(w) = WeightUnit::from_str(&s) {
            return Ok(MetricSystem::Weight(w));
        }

        Err(UnitError::UnknownUnit {
            unit: s.to_string(),
            position: "",
        })
    }
}

fn parse_or_exit(s: &str, args: &'static str) -> MetricSystem {
    MetricSystem::from_str(s)
        .map_err(|_| UnitError::UnknownUnit {
            unit: s.to_string(),
            position: args,
        })
        .unwrap_or_else(|e| handle_error(e))
}

pub fn run_conversion(from_unit: &str, to_unit: &str, value: f64) {
    let from = parse_or_exit(from_unit, &"--from");
    let to = parse_or_exit(to_unit, &"--to");

    match (from, to) {
        (MetricSystem::Tempr(from), MetricSystem::Tempr(to)) => {
            let res = TemperatureUnit::convert(&from, &to, value);
            println!("{value}{from} = {res}{to}");
        }

        (MetricSystem::Len(from), MetricSystem::Len(to)) => {
            let res = LengthUnit::convert(&from, &to, value);
            println!("{value}{from} = {res}{to}");
        }

        (MetricSystem::Weight(from), MetricSystem::Weight(to)) => {
            let res = WeightUnit::convert(&from, &to, value);
            println!("{value}{from} = {res}{to}");
        }

        _ => {
            eprintln!(
                "Error: convert between different unit categories is not permit!\n(--from: {} --to: {})",
                &from_unit, &to_unit
            );
            std::process::exit(1);
        }
    }
}

pub fn run_list() {
    todo!();
}
