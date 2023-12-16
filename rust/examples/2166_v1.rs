// 다각형의 면적
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

struct Vector(i64, i64);

fn get_ccw(p1: &Vector, p2: &Vector, p3: &Vector) -> f64 {
    let Vector(x1, y1) = p1;
    let Vector(x2, y2) = p2;
    let Vector(x3, y3) = p3;
    (x1 * y2 + x2 * y3 + x3 * y1 - (x2 * y1 + x3 * y2 + x1 * y3)) as f64
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i64>);
    let num_v = input.next().unwrap() as usize;
    let mut vectors: Vec<Vector> = Vec::new();
    for _ in 0..num_v {
        vectors.push(Vector(input.next().unwrap(), input.next().unwrap()));
    }
    let mut result = 0.0;
    let fixed_point = &vectors[0];
    for (v1, v2) in vectors.iter().skip(1).zip(vectors.iter().skip(2)) {
        result += get_ccw(fixed_point, v1, v2);
    }
    result /= 2.0;
    result = (result * 100.0).round() / 100.0;
    writeln!(output, "{:.1}", result.abs()).unwrap();
    print!("{}", output);
    Ok(())
}
