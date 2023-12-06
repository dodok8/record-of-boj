use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn get_polygon_inside(polygon_v: &Vec<Vector>, point: &Vector) -> bool {
    let mut count_crossed = 0;
    for idx in 0..polygon_v.len() {
        let jdx = (idx + 1) % polygon_v.len();
        if (point.1 >= polygon_v[idx].1) != (point.1 > polygon_v[jdx].1) {
            let crossed_x: f64 = (polygon_v[jdx].0 - polygon_v[idx].0)
                * (point.1 - polygon_v[idx].1)
                / (polygon_v[jdx].1 - polygon_v[idx].1)
                + polygon_v[idx].0;
            if crossed_x >= point.0 {
                count_crossed += 1;
            }
        }
    }
    count_crossed % 2 != 0
}
struct Vector(f64, f64);
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<f64>);
    let num_v = input.next().unwrap() as usize;
    let mut polygon_v = Vec::new();
    for _ in 0..num_v {
        let v_x = input.next().unwrap();
        let v_y = input.next().unwrap();
        polygon_v.push(Vector(v_x, v_y))
    }
    for _ in 0..3 {
        let v_x = input.next().unwrap();
        let v_y = input.next().unwrap();
        writeln!(
            output,
            "{}",
            if get_polygon_inside(&polygon_v, &Vector(v_x, v_y)) {
                1
            } else {
                0
            }
        )
        .unwrap();
    }
    print!("{}", output);
    Ok(())
}
