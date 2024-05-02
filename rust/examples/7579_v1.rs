// 앱

use std::cmp::max;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let goal = input.next().unwrap();

    let mut memories = vec![0; 1];
    memories.append(&mut input.by_ref().take(n).collect::<Vec<usize>>());
    let mut costs = vec![0; 1];
    costs.append(&mut input.by_ref().take(n).collect::<Vec<usize>>());

    let sum = costs.iter().sum();
    let mut cache = vec![vec![0; sum + 1]; n + 1];

    for idx in 1..=n {
        for jdx in 0..=sum {
            if jdx >= costs[idx] {
                cache[idx][jdx] = max(
                    cache[idx][jdx],
                    cache[idx - 1][jdx - costs[idx]] + memories[idx],
                );
            }

            cache[idx][jdx] = max(cache[idx][jdx], cache[idx - 1][jdx]);
        }
    }
    for idx in 0..=sum {
        if cache[n][idx] >= goal {
            writeln!(output, "{}", idx).unwrap();
            break;
        }
    }
    print!("{}", output);
    Ok(())
}
