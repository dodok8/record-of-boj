// 선분 교차 3

use std::cmp;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

type Point = (i128, i128);

fn get_ccw(p1: &Point, p2: &Point, p3: &Point) -> i128 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let (x3, y3) = p3;
    x1 * y2 + x2 * y3 + x3 * y1 - (x2 * y1 + x3 * y2 + x1 * y3)
}

fn check_crossed(p1: &Point, p2: &Point, p3: &Point, p4: &Point) -> bool {
    let mut crossed = false;
    if p1 == p3 || p1 == p4 || p2 == p3 || p2 == p4 {
        crossed = true;
    } else {
        let ccw_12 = get_ccw(p1, p2, p3) * get_ccw(p1, p2, p4);
        let ccw_34 = get_ccw(p3, p4, p1) * get_ccw(p3, p4, p2);

        if ccw_12 == 0 && ccw_34 == 0 {
            if cmp::min(p1.0, p2.0) <= cmp::max(p3.0, p4.0)
                && cmp::min(p3.0, p4.0) <= cmp::max(p1.0, p2.0)
                && cmp::min(p1.1, p2.1) <= cmp::max(p3.1, p4.1)
                && cmp::min(p3.1, p4.1) <= cmp::max(p1.1, p2.1)
            {
                crossed = true;
            }
        } else {
            crossed = ccw_12 <= 0 && ccw_34 <= 0;
        }
    }

    crossed
}

fn get_intersection(p1: &Point, p2: &Point, p3: &Point, p4: &Point) -> Option<(f64, f64)> {
    let px =
        (p1.0 * p2.1 - p1.1 * p2.0) * (p3.0 - p4.0) - (p1.0 - p2.0) * (p3.0 * p4.1 - p3.1 * p4.0);
    let py =
        (p1.0 * p2.1 - p1.1 * p2.0) * (p3.1 - p4.1) - (p1.1 - p2.1) * (p3.0 * p4.1 - p3.1 * p4.0);

    let p = (p1.0 - p2.0) * (p3.1 - p4.1) - (p1.1 - p2.1) * (p3.0 - p4.0);

    if p == 0 {
        let overlap_x = cmp::max(cmp::min(p1.0, p2.0), cmp::min(p3.0, p4.0))
            < cmp::min(cmp::max(p1.0, p2.0), cmp::max(p3.0, p4.0));
        let overlap_y = cmp::max(cmp::min(p1.1, p2.1), cmp::min(p3.1, p4.1))
            < cmp::min(cmp::max(p1.1, p2.1), cmp::max(p3.1, p4.1));

        if overlap_x || overlap_y {
            return None;
        }

        if p1 == p3 && p2 != p4 {
            return Some((p1.0 as f64, p1.1 as f64));
        }
        if p2 == p3 && p1 != p4 {
            return Some((p2.0 as f64, p2.1 as f64));
        }
        if p1 == p4 && p2 != p3 {
            return Some((p1.0 as f64, p1.1 as f64));
        }
        if p2 == p4 && p1 != p3 {
            return Some((p2.0 as f64, p2.1 as f64));
        }
        None
    } else {
        let x: f64 = (px as f64) / (p as f64);
        let y: f64 = (py as f64) / (p as f64);
        Some((x, y))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i128>);
    let p1 = (input.next().unwrap(), input.next().unwrap());
    let p2 = (input.next().unwrap(), input.next().unwrap());

    let p3 = (input.next().unwrap(), input.next().unwrap());
    let p4 = (input.next().unwrap(), input.next().unwrap());

    let crossed = check_crossed(&p1, &p2, &p3, &p4);

    if crossed {
        writeln!(output, "1")?;
        if let Some(val) = get_intersection(&p1, &p2, &p3, &p4) {
            writeln!(output, "{:.9} {:.9}", val.0, val.1)?;
        }
    } else {
        writeln!(output, "0")?;
    }
    print!("{}", output);
    Ok(())
}
