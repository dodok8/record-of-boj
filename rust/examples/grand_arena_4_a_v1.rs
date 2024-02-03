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
    let mut sorted = vec![0];
    for idx in 0..n {
        origin.push(input.next().unwrap());
        sorted.push((idx + 1) as u128);
    }

    let mut cache = vec![vec![0_usize; n + 1]; n + 1];
    let mut final_contained = vec![];
    for (idx, o_num) in (0..).zip(origin.clone()).skip(1) {
        for (jdx, s_num) in (0..).zip(sorted.clone()).skip(1) {
            if origin[idx - 1] > o_num {
                if o_num == s_num {
                    cache[jdx][idx] = 1;
                } else {
                    cache[jdx][idx] = 0;
                }
            } else if o_num == s_num {
                cache[jdx][idx] = cache[jdx - 1][idx - 1] + 1;
            } else {
                cache[jdx][idx] = *[cache[jdx - 1][idx], cache[jdx][idx - 1]]
                    .iter()
                    .max()
                    .unwrap();
            }

            if o_num == s_num && (idx == n || o_num > origin[idx + 1]) {
                final_contained.push(cache[jdx][idx]);
            }
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
