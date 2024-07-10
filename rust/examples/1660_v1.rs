// 캡틴 이다솜

use std::cmp::min;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();

    let mut counts = vec![0];

    let mut answers = vec![0usize; n + 1];
    for idx in 1..=n {
        let count = counts[idx - 1] + idx * (idx + 1) / 2;
        if count > n {
            break;
        } else {
            counts.push(count);
            answers[count] = 1;
        }
    }

    for idx in 1..=n {
        match answers[idx] {
            1 => continue,
            _ => {
                let mut ans = usize::MAX;

                for &p_idx in counts.iter().skip(1) {
                    if idx < p_idx {
                        break;
                    }
                    ans = min(ans, answers[idx - p_idx] + answers[p_idx]);
                }
                answers[idx] = ans;
            }
        }
    }
    writeln!(output, "{}", answers[n]).unwrap();
    print!("{}", output);
    Ok(())
}
