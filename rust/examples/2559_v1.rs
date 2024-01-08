use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let mut sums_temp = vec![0];
    let n = input.next().unwrap() as usize;
    let k = input.next().unwrap() as usize;
    for idx in 1..(n + 1) {
        sums_temp.push(sums_temp[idx - 1] + input.next().unwrap());
    }

    let mut ans = i64::MIN;
    for idx in 1..(n + 1) {
        let jdx = idx + k - 1;
        if jdx > n {
            break;
        } else {
            let sum = sums_temp[jdx] - sums_temp[idx - 1];

            if sum > ans {
                ans = sum;
            }
        }
    }
    writeln!(output, "{}", ans).unwrap();
    print!("{}", output);
    Ok(())
}
