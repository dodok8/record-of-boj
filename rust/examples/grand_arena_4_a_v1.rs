// 정렬된 연속한 부분 수열의 개수

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<u128>);
    let n = input.next().unwrap() as usize;

    let mut origin = vec![0];
    for idx in 0..n {
        origin.push(input.next().unwrap());
    }

    let mut cache = vec![0; n + 1];
    let mut final_contained = vec![];
    for (idx, &o_num) in origin.iter().enumerate().skip(1) {
        if o_num > origin[idx - 1] {
            cache[idx] = cache[idx - 1] + 1;
        } else {
            cache[idx] = 1;
        }

        if idx == n || origin[idx + 1] < o_num {
            final_contained.push(cache[idx]);
        }
    }
    writeln!(
        output,
        "{}",
        final_contained
            .iter()
            .map(|x| (1..=*x).product::<usize>())
            .sum::<usize>()
    )
    .unwrap();
    print!("{}", output);
    Ok(())
}
