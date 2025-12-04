mod error;
pub mod length;
pub mod temperature;
pub mod weight;

use std::str::FromStr;

use length::LengthUnit;
use temperature::TemperatureUnit;
use weight::WeightUnit;

use crate::history::{History, save_history};

use self::error::{UnitError, handle_error};

#[derive(Debug)]
enum MetricSystem {
    Tempr(TemperatureUnit),
    Len(LengthUnit),
    Weight(WeightUnit),
}

impl FromStr for MetricSystem {
    type Err = UnitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(t) = TemperatureUnit::from_str(s) {
            return Ok(MetricSystem::Tempr(t));
        }

        if let Ok(l) = LengthUnit::from_str(s) {
            return Ok(MetricSystem::Len(l));
        }

        if let Ok(w) = WeightUnit::from_str(s) {
            return Ok(MetricSystem::Weight(w));
        }

        Err(UnitError::UnknownUnit {
            unit: s.to_string(),
            position: "",
        })
    }
}

impl MetricSystem {
    fn category_name(&self) -> &'static str {
        match self {
            MetricSystem::Tempr(_) => "Temperature",
            MetricSystem::Len(_) => "Length",
            MetricSystem::Weight(_) => "Weight",
        }
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
    let from = parse_or_exit(from_unit, "--from");
    let to = parse_or_exit(to_unit, "--to");

    match (&from, &to) {
        (MetricSystem::Tempr(from), MetricSystem::Tempr(to)) => {
            let res = TemperatureUnit::convert(from, to, value);
            println!("{value} {from} = {res} {to}");
            save_history(History {
                from_unit: from.as_str().to_string(),
                to_unit: to.as_str().to_string(),
                value_unit: value,
                output_unit: res,
            });
        }

        (MetricSystem::Len(from), MetricSystem::Len(to)) => {
            if value < 0.0 {
                eprintln!("Error: `{}` is invalid value for Length Unit!", value);
                std::process::exit(1)
            }
            let res = LengthUnit::convert(from, to, value);
            println!("{value} {from} = {res} {to}");
            save_history(History {
                from_unit: from.as_str().to_string(),
                to_unit: to.as_str().to_string(),
                value_unit: value,
                output_unit: res,
            });
        }

        (MetricSystem::Weight(from), MetricSystem::Weight(to)) => {
            if value < 0.0 {
                eprintln!("Error: `{}` is invalid value for Weight Unit!", value);
                std::process::exit(1)
            }
            let res = WeightUnit::convert(from, to, value);
            println!("{value} {from} = {res} {to}");
            save_history(History {
                from_unit: from.as_str().to_string(),
                to_unit: to.as_str().to_string(),
                value_unit: value,
                output_unit: res,
            });
        }

        _ => {
            eprintln!("Error: convert between different unit categories is not permit!");
            eprintln!("--from: {} [{}]", from_unit, &from.category_name());
            eprintln!("--to: {} [{}]", to_unit, &to.category_name());
            std::process::exit(1);
        }
    }
}

pub fn run_list() {
    println!("List all units:");

    println!("  Temperature:");
    for t in TemperatureUnit::ALL {
        println!("      - {:?} ({})", t, &t.to_string());
    }

    println!("\n  Length:");
    for l in LengthUnit::ALL {
        println!("      - {:?} ({})", l, &l.to_string().to_lowercase());
    }

    println!("\n  Weight:");
    for w in WeightUnit::ALL {
        println!("      - {:?} ({})", w, &w.to_string().to_lowercase());
    }
}
