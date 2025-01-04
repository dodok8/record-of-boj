use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let x = input.next().unwrap();
    let y = input.next().unwrap();
    let c = input.next().unwrap();

    let dist = ((x as f64).powi(2) + (y as f64).powi(2)).sqrt();
    let c = c as f64;
    
    let ans = if dist == 0.0 {
        0
    } else if (dist - c).abs() < 1e-10 {
        1
    } else if dist < c {
        2
    } else {
        let min_moves = (dist / c).ceil() as i64;
        if min_moves % 2 == 0 {
            min_moves
        } else {
            min_moves + 1
        }
    };

    writeln!(output, "{}", ans)?;
    print!("{}", output);
    Ok(())
}