// 지민이의 테러
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn get_ccw(p1: &Vector, p2: &Vector, p3: &Vector) -> i64 {
    let Vector(x1, y1) = p1;
    let Vector(x2, y2) = p2;
    let Vector(x3, y3) = p3;
    x1 * y2 + x2 * y3 + x3 * y1 - (x2 * y1 + x3 * y2 + x1 * y3)
}

fn get_polygon_inside(polygon_v: &Vec<Vector>, point: &Vector) -> bool {
    let mut count_crossed = 0;
    for idx in 0..polygon_v.len() {
        let jdx = (idx + 1) % polygon_v.len();
        if get_ccw(&polygon_v[idx], &polygon_v[jdx], point) == 0
            && (point.0 > polygon_v[idx].0) != (point.0 > polygon_v[jdx].0)
        {
            return true;
        }
        if (point.1 > polygon_v[idx].1) != (point.1 > polygon_v[jdx].1) {
            let crossed_x: f64 = (polygon_v[jdx].0 - polygon_v[idx].0) as f64
                * (point.1 - polygon_v[idx].1) as f64
                / (polygon_v[jdx].1 - polygon_v[idx].1) as f64
                + polygon_v[idx].0 as f64;
            if crossed_x > point.0 as f64 {
                count_crossed += 1;
            }
        }
    }
    count_crossed % 2 != 0
}
struct Vector(i64, i64);
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i64>);
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
