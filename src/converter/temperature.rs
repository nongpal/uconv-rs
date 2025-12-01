/// reference https://doc.rust-lang.org/book/ch20-02-advanced-traits.html
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug)]
pub enum UnitError {
    ParsingError(String),
}

struct ConversionResult<T> {
    from_unit: T,
    to_unit: T,
    from_value: f64,
    to_value: f64,
}

#[derive(Clone, Copy)]
pub enum TemperatureUnit {
    Celcius,
    Fahrenheit,
    Kelvin,
}

#[derive(Clone, Copy)]
pub enum LengthUnit {
    Kilometer,
    Hectometer,
    Decameter,
    Meter,
    Decimeter,
    Centimeter,
    Milimeter,
}

#[derive(Clone, Copy)]
pub enum WeightUnit {
    Ton,
    Kilogram,
    Hectogram,
    Decagram,
    Gram,
    Decigram,
    Centigram,
    Miligram,
}

pub trait Conversion {
    fn convert(from_unit: &Self, to_unit: &Self, value: f64) -> f64;
}

impl LengthUnit {
    fn ratio(&self) -> f64 {
        match self {
            LengthUnit::Milimeter => 0.001,
            LengthUnit::Centimeter => 0.01,
            LengthUnit::Decimeter => 0.1,
            LengthUnit::Meter => 1.0,
            LengthUnit::Decameter => 10.0,
            LengthUnit::Hectometer => 100.0,
            LengthUnit::Kilometer => 1000.0,
        }
    }
}

impl WeightUnit {
    fn ratio(&self) -> f64 {
        match self {
            WeightUnit::Miligram => 0.001,
            WeightUnit::Centigram => 0.01,
            WeightUnit::Decigram => 0.1,
            WeightUnit::Gram => 1.0,
            WeightUnit::Decagram => 10.0,
            WeightUnit::Hectogram => 100.0,
            WeightUnit::Kilogram => 1000.0,
            WeightUnit::Ton => 1000000.0,
        }
    }
}

impl FromStr for TemperatureUnit {
    type Err = UnitError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "celcius" => Ok(TemperatureUnit::Celcius),
            "fahrenheit" => Ok(TemperatureUnit::Fahrenheit),
            "kelvin" => Ok(TemperatureUnit::Kelvin),
            _ => Err(UnitError::ParsingError(format!("Unkown unit {}", s))),
        }
    }
}

impl FromStr for LengthUnit {
    type Err = UnitError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "kilometer" => Ok(LengthUnit::Kilometer),
            "km" => Ok(LengthUnit::Kilometer),
            "hectometer" => Ok(LengthUnit::Hectometer),
            "hm" => Ok(LengthUnit::Hectometer),
            "decameter" => Ok(LengthUnit::Decameter),
            "dam" => Ok(LengthUnit::Decameter),
            "meter" => Ok(LengthUnit::Meter),
            "m" => Ok(LengthUnit::Meter),
            "decimeter" => Ok(LengthUnit::Decimeter),
            "dm" => Ok(LengthUnit::Decimeter),
            "centimeter" => Ok(LengthUnit::Centimeter),
            "cm" => Ok(LengthUnit::Centimeter),
            "milimeter" => Ok(LengthUnit::Milimeter),
            "mm" => Ok(LengthUnit::Milimeter),
            _ => Err(UnitError::ParsingError(format!("Unkown unit {}", s))),
        }
    }
}

impl FromStr for WeightUnit {
    type Err = UnitError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ton" => Ok(WeightUnit::Ton),
            "t" => Ok(WeightUnit::Ton),
            "kilogram" => Ok(WeightUnit::Kilogram),
            "kg" => Ok(WeightUnit::Kilogram),
            "hectogram" => Ok(WeightUnit::Hectogram),
            "hg" => Ok(WeightUnit::Hectogram),
            "decagram" => Ok(WeightUnit::Decagram),
            "dag" => Ok(WeightUnit::Decagram),
            "gram" => Ok(WeightUnit::Gram),
            "gr" => Ok(WeightUnit::Gram),
            "decigram" => Ok(WeightUnit::Decigram),
            "dg" => Ok(WeightUnit::Decigram),
            "centigram" => Ok(WeightUnit::Centigram),
            "cg" => Ok(WeightUnit::Centigram),
            "miligram" => Ok(WeightUnit::Miligram),
            "mg" => Ok(WeightUnit::Miligram),
            _ => Err(UnitError::ParsingError(format!("Unkown unit {}", s))),
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

impl Display for LengthUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            LengthUnit::Kilometer => "Km",
            LengthUnit::Hectometer => "Hm",
            LengthUnit::Decameter => "Dam",
            LengthUnit::Meter => "M",
            LengthUnit::Decimeter => "Dm",
            LengthUnit::Centimeter => "Cm",
            LengthUnit::Milimeter => "Mm",
        };

        write!(f, "{}", s)
    }
}

impl Display for WeightUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            WeightUnit::Ton => "T",
            WeightUnit::Kilogram => "Kg",
            WeightUnit::Hectogram => "Hg",
            WeightUnit::Decagram => "Dag",
            WeightUnit::Gram => "gr",
            WeightUnit::Decigram => "Dg",
            WeightUnit::Centigram => "Cg",
            WeightUnit::Miligram => "Mg",
        };

        write!(f, "{}", s)
    }
}

impl Conversion for TemperatureUnit {
    fn convert(from_unit: &Self, to_unit: &Self, value: f64) -> f64 {
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

impl Conversion for LengthUnit {
    fn convert(from_unit: &Self, to_unit: &Self, value: f64) -> f64 {
        (value * from_unit.ratio()) / to_unit.ratio()
    }
}

impl Conversion for WeightUnit {
    fn convert(from_unit: &Self, to_unit: &Self, value: f64) -> f64 {
        (value * from_unit.ratio()) / to_unit.ratio()
    }
}

/// psuedo-assert-like rust
pub fn custom_assert_approx(a: f64, b: f64) {
    let eps = 0.0001;
    let x = a - b;
    let dif = if x > 0.0 { x } else { -x };

    if dif >= eps {
        panic!("Result is not similar enough");
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
