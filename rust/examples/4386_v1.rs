// 별자리 만들기

use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

struct TotalOrdF64(f64);

impl PartialEq for TotalOrdF64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for TotalOrdF64 {}

impl PartialOrd for TotalOrdF64 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TotalOrdF64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

fn get_d(point1: &(f64, f64), point2: &(f64, f64)) -> f64 {
    let (x1, y1) = point1;
    let (x2, y2) = point2;
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<f64>);

    let n: usize = input.next().unwrap() as usize;
    let mut points = Vec::new();
    for _ in 0..n {
        let x = input.next().unwrap();
        let y = input.next().unwrap();
        points.push((x, y));
    }
    let mut contained = vec![false; n];
    let mut edges_pq = BinaryHeap::new();
    let mut dist = 0.0;
    edges_pq.push(Reverse((TotalOrdF64(0.0), 0_usize)));
    while !edges_pq.is_empty() {
        let Reverse((TotalOrdF64(curr_d), curr_v)) = edges_pq.pop().unwrap();
        if contained[curr_v] {
            continue;
        }
        contained[curr_v] = true;
        dist += curr_d;
        for (idx, point) in points.iter().enumerate() {
            if !contained[idx] {
                edges_pq.push(Reverse((TotalOrdF64(get_d(point, &points[curr_v])), idx)));
            }
        }
    }
    writeln!(output, "{:.2}", dist).unwrap();
    print!("{}", output);
    Ok(())
}
