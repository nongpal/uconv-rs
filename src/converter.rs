pub fn celcius_to_fahrenheit(celc: f64) -> f64 {
    (celc * 9.0 / 5.0) + 32.0
}

pub fn celcius_to_kelvin(celc: f64) -> f64 {
    celc + 273.15
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celc2fah() {
        assert_eq!(celcius_to_fahrenheit(0.0), 32.0);
    }

    #[test]
    fn test_celc2kel() {
        assert_eq!(celcius_to_kelvin(0.0), 273.15);
    }
}
