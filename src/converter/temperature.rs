/// reference https://doc.rust-lang.org/book/ch20-02-advanced-traits.html
use std::fmt::Display;
use std::str::FromStr;

use super::error::{UnitError, custom_assert_approx};

#[derive(Clone, Copy, Debug)]
pub enum TemperatureUnit {
    Celcius,
    Fahrenheit,
    Kelvin,
}

impl FromStr for TemperatureUnit {
    type Err = UnitError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "celcius" => Ok(TemperatureUnit::Celcius),
            "fahrenheit" => Ok(TemperatureUnit::Fahrenheit),
            "kelvin" => Ok(TemperatureUnit::Kelvin),
            _ => Err(UnitError::UnknownUnit {
                unit: s.to_string(),
                position: "",
            }),
        }
    }
}

impl Display for TemperatureUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TemperatureUnit::Celcius => "°C",
            TemperatureUnit::Fahrenheit => "°F",
            TemperatureUnit::Kelvin => "K",
        };

        write!(f, "{}", s)
    }
}

impl TemperatureUnit {
    pub fn convert(from_unit: &Self, to_unit: &Self, value: f64) -> f64 {
        let to_kelvin = match from_unit {
            TemperatureUnit::Celcius => value + 273.15,
            TemperatureUnit::Fahrenheit => (value - 32.0) * (5.0 / 9.0) + 273.15,
            TemperatureUnit::Kelvin => value,
        };

        match to_unit {
            TemperatureUnit::Celcius => to_kelvin - 273.15,
            TemperatureUnit::Fahrenheit => (to_kelvin - 273.15) * (9.0 / 5.0) + 32.0,
            TemperatureUnit::Kelvin => to_kelvin,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celcius_to_fahrenheit() {
        let from = TemperatureUnit::from_str("celcius").unwrap();
        let to = TemperatureUnit::from_str("fahrenheit").unwrap();
        let result = TemperatureUnit::convert(&from, &to, 0.0);
        custom_assert_approx(result, 32.0);
    }
}
