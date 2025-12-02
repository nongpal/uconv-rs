use std::fmt;
use std::str;

use super::error::UnitError;

#[derive(Clone, Copy, Debug)]
pub enum WeightUnit {
    Tonne,
    Kilogram,
    Hectogram,
    Decagram,
    Gram,
    Decigram,
    Centigram,
    Miligram,
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
            WeightUnit::Tonne => 1000000.0,
        }
    }

    pub fn convert(from_unit: &Self, to_unit: &Self, value: f64) -> f64 {
        (value * from_unit.ratio()) / to_unit.ratio()
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Tonne => "tonne",
            Self::Kilogram => "kilogram",
            Self::Hectogram => "hectogram",
            Self::Decagram => "decagram",
            Self::Gram => "gram",
            Self::Decigram => "decigram",
            Self::Centigram => "centigram",
            Self::Miligram => "miligram",
        }
    }

    pub const ALL: [Self; 8] = [
        Self::Tonne,
        Self::Kilogram,
        Self::Hectogram,
        Self::Decagram,
        Self::Gram,
        Self::Decigram,
        Self::Centigram,
        Self::Miligram,
    ];
}

impl str::FromStr for WeightUnit {
    type Err = UnitError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "tonne" => Ok(WeightUnit::Tonne),
            "t" => Ok(WeightUnit::Tonne),
            "kilogram" => Ok(WeightUnit::Kilogram),
            "kg" => Ok(WeightUnit::Kilogram),
            "hectogram" => Ok(WeightUnit::Hectogram),
            "hg" => Ok(WeightUnit::Hectogram),
            "decagram" => Ok(WeightUnit::Decagram),
            "dag" => Ok(WeightUnit::Decagram),
            "gram" => Ok(WeightUnit::Gram),
            "g" => Ok(WeightUnit::Gram),
            "decigram" => Ok(WeightUnit::Decigram),
            "dg" => Ok(WeightUnit::Decigram),
            "centigram" => Ok(WeightUnit::Centigram),
            "cg" => Ok(WeightUnit::Centigram),
            "miligram" => Ok(WeightUnit::Miligram),
            "mg" => Ok(WeightUnit::Miligram),
            _ => Err(UnitError::UnknownUnit {
                unit: s.to_string(),
                position: "",
            }),
        }
    }
}

impl fmt::Display for WeightUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            WeightUnit::Tonne => "T",
            WeightUnit::Kilogram => "Kg",
            WeightUnit::Hectogram => "Hg",
            WeightUnit::Decagram => "Dag",
            WeightUnit::Gram => "g",
            WeightUnit::Decigram => "Dg",
            WeightUnit::Centigram => "Cg",
            WeightUnit::Miligram => "Mg",
        };

        write!(f, "{}", s)
    }
}
