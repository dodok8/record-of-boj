use std::cmp::{max, min};
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

struct Range {
    t: (i64, i64),
    f: (i64, i64),
}

fn get_intersection(curr: &(i64, i64), other: &(i64, i64)) -> Option<(i64, i64)> {
    let start = std::cmp::max(curr.0, other.0);
    let end = std::cmp::min(curr.1, other.1);
    if start > end {
        None
    } else {
        Some((start, end))
    }
}

fn is_contain(curr: &(i64, i64), num: i64) -> bool {
    curr.0 <= num && num <= curr.1
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i64>);
    let n = input.next().unwrap();
    let mut ranges: Vec<Range> = Vec::new();
    for _ in 0..n {
        let k = input.next().unwrap();
        if k > 0 {
            ranges.push(Range {
                t: (k, n),
                f: (0, k - 1),
            });
        } else {
            let k = -k;
            ranges.push(Range {
                t: (0, k),
                f: (k + 1, n),
            });
        }
    }
    let mut results = Vec::new();
    let mut travel_stack = VecDeque::new();
    travel_stack.push_back(((0, 0), ranges[0].t));
    travel_stack.push_back(((0_usize, 1_i64), ranges[0].f));
    while !travel_stack.is_empty() {
        let ((curr_idx, num_lie), curr_range) = travel_stack.pop_back().unwrap();
        if curr_idx == (n - 1) as usize {
            if is_contain(&curr_range, num_lie) {
                results.push(num_lie);
            }
            continue;
        }
        if num_lie <= curr_range.1 {
            if let Some(next_range) = get_intersection(&curr_range, &ranges[curr_idx + 1].t) {
                travel_stack.push_back(((curr_idx + 1, num_lie), next_range));
            }
        }

        if num_lie < curr_range.1 {
            if let Some(next_range) = get_intersection(&curr_range, &ranges[curr_idx + 1].f) {
                travel_stack.push_back(((curr_idx + 1, num_lie + 1), next_range));
            }
        }
    }
    results.sort();
    writeln!(output, "{}", results.len()).unwrap();
    for num in results {
        write!(output, "{} ", num).unwrap();
    }
    println!("{}", output);
    Ok(())
}
