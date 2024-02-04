use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let grade = input.next().unwrap();
    match grade {
        "A+" => writeln!(output, "4.3").unwrap(),
        "A0" => writeln!(output, "4.0").unwrap(),
        "A-" => writeln!(output, "3.7").unwrap(),
        "B+" => writeln!(output, "3.3").unwrap(),
        "B0" => writeln!(output, "3.0").unwrap(),
        "B-" => writeln!(output, "2.7").unwrap(),
        "C+" => writeln!(output, "2.3").unwrap(),
        "C0" => writeln!(output, "2.0").unwrap(),
        "C-" => writeln!(output, "1.7").unwrap(),
        "D+" => writeln!(output, "1.3").unwrap(),
        "D0" => writeln!(output, "1.0").unwrap(),
        "D-" => writeln!(output, "0.7").unwrap(),
        "F" => writeln!(output, "0.0").unwrap(),
        _ => panic!(),
    }

    print!("{}", output);
    Ok(())
}
