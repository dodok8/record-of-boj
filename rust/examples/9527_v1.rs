// 1의 개수 세기

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let start = input.next().unwrap();
    let end = input.next().unwrap();

    let mut count_one = vec![0; end + 1];
    count_one[1] = 1;
    count_one[2] = 1;
    let mut result = 0;
    for idx in 0..=end {
        if idx >= 3 {
            let log2 = (idx as f32 + 1.).log2().ceil() as usize - 1;
            let pow = 2_usize.pow(log2 as u32);

            count_one[idx] = count_one[idx - pow] + 1;
        }
        if idx >= start {
            result += count_one[idx];
        }
    }
    writeln!(output, "{}", result).unwrap();
    print!("{}", output);
    Ok(())
}
