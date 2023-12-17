// CCW

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn get_ccw(p1: &Vector, p2: &Vector, p3: &Vector) -> i64 {
    let Vector(x1, y1) = p1;
    let Vector(x2, y2) = p2;
    let Vector(x3, y3) = p3;
    x1 * y2 + x2 * y3 + x3 * y1 - (x2 * y1 + x3 * y2 + x1 * y3)
}
struct Vector(i64, i64);

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i64>);
    let p1 = Vector(input.next().unwrap(), input.next().unwrap());
    let p2 = Vector(input.next().unwrap(), input.next().unwrap());
    let p3 = Vector(input.next().unwrap(), input.next().unwrap());
    let ccw = get_ccw(&p1, &p2, &p3);
    match ccw.cmp(&0) {
        std::cmp::Ordering::Greater => writeln!(output, "1").unwrap(),
        std::cmp::Ordering::Less => writeln!(output, "-1").unwrap(),
        std::cmp::Ordering::Equal => writeln!(output, "0").unwrap(),
    }

    print!("{}", output);
    Ok(())
}
