// 정사각형
use std::fmt::Write;
use std::io::{stdin, Read};

struct Point2D(i32, i32);

fn get_square_distance(point1: &Point2D, point2: &Point2D) -> i32 {
    i32::pow(point1.0 - point2.0, 2) + i32::pow(point1.1 - point2.1, 2)
}

fn main() {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let test_case: i32 = input.next().unwrap();
    for _ in 0..test_case {
        let mut points: Vec<Point2D> = Vec::new();
        let mut edges: Vec<i32> = Vec::new();
        for _ in 0..4 {
            let x: i32 = input.next().unwrap();
            let y: i32 = input.next().unwrap();
            points.push(Point2D(x, y))
        }
        for idx in 0..4 {
            for jdx in idx + 1..4 {
                edges.push(get_square_distance(&points[idx], &points[jdx]));
            }
        }

        edges.sort();

        if edges[5] != edges[4] || edges[0] != edges[2] || edges[2] != edges[3] {
            writeln!(output, "0").unwrap();
        } else {
            writeln!(output, "1").unwrap();
        }
    }

    print!("{}", output)
}
