// Ax+Bsin(x)=C ②

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn get_newton_value(x_n: f64, a: f64, b: f64, c: f64) -> f64 {
    let curr_f = a * x_n + b * f64::sin(x_n) - c;
    let divisor = a + b * f64::cos(x_n);

    x_n - (curr_f / divisor)
}

fn get_abs_err(real: f64, detect: f64) -> f64 {
    f64::abs(real - detect)
}

fn get_rel_err(real: f64, detect: f64) -> f64 {
    f64::abs(real - detect) / real
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<f64>);

    let a = input.next().unwrap();
    let b = input.next().unwrap();
    let c = input.next().unwrap();

    let mut x_curr = 10.0;

    let ans = loop {
        let x_next = get_newton_value(x_curr, a, b, c);

        if get_abs_err(x_next, x_curr) < f64::powf(10.0, -9.0)
            && get_rel_err(x_next, x_curr) < f64::powf(10.0, -9.0)
        {
            break x_next;
        }
        x_curr = x_next;
    };

    writeln!(output, "{}", ans).unwrap();
    print!("{}", output);
    Ok(())
}
