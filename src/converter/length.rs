use super::error::UnitError;
use std::fmt;
use std::str;

#[derive(Clone, Copy, Debug)]
pub enum LengthUnit {
    Kilometer,
    Hectometer,
    Decameter,
    Meter,
    Decimeter,
    Centimeter,
    Milimeter,
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

    pub fn convert(from_unit: &Self, to_unit: &Self, value: f64) -> f64 {
        (value * from_unit.ratio()) / to_unit.ratio()
    }
}

impl str::FromStr for LengthUnit {
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
            _ => Err(UnitError::UnknownUnit {
                unit: s.to_string(),
                position: "",
            }),
        }
    }
}

impl fmt::Display for LengthUnit {
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
