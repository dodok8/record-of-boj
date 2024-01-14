// 모바일 광고 입찰

use std::collections::HashMap;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let num_ad = input.next().unwrap();
    let goal_ad = input.next().unwrap();
    let mut curr_ad = 0;
    let mut count_deltas = HashMap::new();
    for _ in 0..num_ad {
        let a = input.next().unwrap();
        let b = input.next().unwrap();
        let delta = b - a;
        if delta > 0 {
            let count = count_deltas.entry(delta).or_insert(0);
            *count += 1;
        } else {
            curr_ad += 1;
        }
    }

    if curr_ad >= goal_ad {
        writeln!(output, "0").unwrap();
    } else {
        let mut previous_key: Option<i64> = None;

        let mut keys: Vec<i64> = count_deltas.keys().cloned().collect();
        keys.sort_unstable();

        for &key in keys.iter() {
            if let Some(prev_key) = previous_key {
                let prev_count = *count_deltas.get(&prev_key).unwrap();
                *count_deltas.entry(key).or_insert(0) += prev_count;
            }
            if *count_deltas.get(&key).unwrap() >= goal_ad - curr_ad {
                writeln!(output, "{}", key).unwrap();
                break;
            }

            previous_key = Some(key);
        }
    }
    print!("{}", output);
    Ok(())
}
