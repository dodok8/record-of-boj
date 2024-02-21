// 선분 교차 2

use std::error::Error;
use std::io::{stdin, Read};

type Point = (i64, i64);

fn get_ccw(p1: &Point, p2: &Point, p3: &Point) -> i64 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let (x3, y3) = p3;
    x1 * y2 + x2 * y3 + x3 * y1 - (x2 * y1 + x3 * y2 + x1 * y3)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i64>);
    let p1 = (input.next().unwrap(), input.next().unwrap());
    let p2 = (input.next().unwrap(), input.next().unwrap());

    let p3 = (input.next().unwrap(), input.next().unwrap());
    let p4 = (input.next().unwrap(), input.next().unwrap());

    let mut crossed = false;
    if p1 == p3 || p1 == p4 || p2 == p3 || p2 == p4 {
        crossed = true;
    } else {
        let ccw_12 = get_ccw(&p1, &p2, &p3) * get_ccw(&p1, &p2, &p4);
        let ccw_34 = get_ccw(&p3, &p4, &p1) * get_ccw(&p3, &p4, &p2);

        if ccw_12 == 0 && ccw_34 == 0 {
            if (p1.0 - p3.0) * (p1.0 - p4.0) < 0 || (p2.0 - p3.0) * (p2.0 - p4.0) < 0 {
                crossed = true;
            }
            if (p1.1 - p3.1) * (p1.1 - p4.1) < 0 || (p2.1 - p3.1) * (p2.1 - p4.1) < 0 {
                crossed = true;
            }
        } else {
            crossed = ccw_12 <= 0 && ccw_34 <= 0;
        }
    }

    if crossed {
        println!("1");
    } else {
        println!("0");
    }

    Ok(())
}
