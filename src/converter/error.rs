#[derive(Debug)]
pub enum UnitError {
    UnknownUnit {
        unit: String,
        position: &'static str,
    },
}

pub fn handle_error(err: UnitError) -> ! {
    match err {
        UnitError::UnknownUnit { unit, position } => {
            eprintln!("Error: [UnknownUnit] \"{}\" at \"{}\"", unit, position);
            eprintln!("Try to check the available unit using \"unitconv list\" instead.");
        }
    }
    std::process::exit(1)
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
