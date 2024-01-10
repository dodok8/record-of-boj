// 개똥벌레
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let max_h = input.next().unwrap();
    let mut jong = vec![0; max_h];
    let mut sucksun = vec![0; max_h];

    for idx in 0..n {
        let curr_h = input.next().unwrap();
        if idx % 2 == 0 {
            sucksun[curr_h - 1] += 1;
        } else {
            jong[curr_h - 1] += 1;
        }
    }

    for idx in (1..max_h).rev() {
        jong[idx - 1] += jong[idx];
        sucksun[idx - 1] += sucksun[idx];
    }

    let mut result = n;
    let mut count = 0;
    for h in 0..max_h {
        let curr = sucksun[h] + jong[max_h - h - 1];

        match result.cmp(&curr) {
            std::cmp::Ordering::Equal => count += 1,
            std::cmp::Ordering::Greater => {
                result = curr;
                count = 1;
            }
            _ => (),
        }
    }
    writeln!(output, "{} {}", result, count).unwrap();
    print!("{}", output);
    Ok(())
}
