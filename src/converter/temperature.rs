pub fn celcius_to_fahrenheit(celc: f64) -> f64 {
    (celc * 9.0 / 5.0) + 32.0
}

pub fn celcius_to_kelvin(celc: f64) -> f64 {
    celc + 273.15
}

pub fn fahrenheit_to_celcius(fahr: f64) -> f64 {
    (fahr - 32.0) * (5.0 / 9.0)
}

pub fn fahrenheit_to_kelvin(fahr: f64) -> f64 {
    fahrenheit_to_celcius(fahr) + 273.15
}

pub fn kelvin_to_celcius(kelv: f64) -> f64 {
    kelv - 273.15
}

pub fn kelvin_to_fahreinheit(kelv: f64) -> f64 {
    celcius_to_fahrenheit(kelv - 273.15)
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
        custom_assert_approx(celcius_to_fahrenheit(0.0), 32.0);
    }

    #[test]
    fn test_celcius_to_kelvin() {
        custom_assert_approx(celcius_to_kelvin(0.0), 273.15);
    }

    #[test]
    fn test_fahrenheit_to_celcius() {
        custom_assert_approx(fahrenheit_to_celcius(32.0), 0.0);
    }

    #[test]
    fn test_fahrenheit_to_kelvin() {
        custom_assert_approx(fahrenheit_to_kelvin(32.0), 273.15);
    }

    #[test]
    fn test_kelv2celc() {
        custom_assert_approx(kelvin_to_celcius(0.0), -273.15);
    }

    #[test]
    fn test_kelv2fah() {
        custom_assert_approx(kelvin_to_fahreinheit(0.0), -459.67);
    }

    #[test]
    fn test_celc_to_fah_to_celc() {
        let value = 0.0;
        let f = celcius_to_fahrenheit(value);
        let c = fahrenheit_to_celcius(f);
        custom_assert_approx(c, 0.0);
    }
}
