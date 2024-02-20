// 타일 채우기

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn get_count(x: usize, cache: &mut Vec<Option<usize>>) -> usize {
    if let Some(val) = cache[x] {
        val
    } else {
        let mut curr_x = get_count(x - 1, cache) * get_count(1, cache);
        let mut sum = 0;
        for val in 0..(x - 1) {
            sum += get_count(val, cache) * 2;
        }
        curr_x += sum;

        cache[x] = Some(curr_x);
        curr_x
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    if n % 2 == 1 {
        writeln!(output, "0").unwrap();
    } else {
        let n = n / 2;
        let mut cache = vec![None; n + 1];
        cache[0] = Some(1_usize);
        cache[1] = Some(3);

        writeln!(output, "{}", get_count(n, &mut cache)).unwrap();
    }
    print!("{}", output);
    Ok(())
}
