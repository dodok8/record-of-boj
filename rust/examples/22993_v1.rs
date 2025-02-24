// 서든어택 3

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let _n = input.next().unwrap();
    let mut curr = input.next().unwrap();

    let mut nums = input.collect::<Vec<usize>>();
    nums.sort_unstable();

    let mut last_survived = true;
    for num in nums {
        if num >= curr {
            last_survived = false;
            break;
        } else {
            curr += num;
        }
    }

    if last_survived {
        writeln!(output, "Yes")?;
    } else {
        writeln!(output, "No")?;
    }
    print!("{}", output);
    Ok(())
}
