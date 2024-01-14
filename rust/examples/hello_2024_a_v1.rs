// 모바일 광고 입찰

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let num_ad = input.next().unwrap();
    let goal_ad = input.next().unwrap();
    let mut curr_ad = 0;
    let mut count_deltas = vec![0; usize::pow(10, 9) + 1];
    for _ in 0..num_ad {
        let a = input.next().unwrap();
        let b = input.next().unwrap();

        if b - a > 0 {
            count_deltas[(b - a) as usize] += 1;
        } else {
            curr_ad += 1;
        }
    }

    if curr_ad >= goal_ad {
        writeln!(output, "0").unwrap();
    } else {
        for idx in 1..(usize::pow(10, 9) + 1) {
            count_deltas[idx] += count_deltas[idx - 1];
            if count_deltas[idx] >= goal_ad - curr_ad {
                writeln!(output, "{}", idx).unwrap();
                break;
            }
        }
    }
    print!("{}", output);
    Ok(())
}
