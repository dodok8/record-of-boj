// 1의 개수 세기

use std::error::Error;
use std::io::{stdin, Read};

fn count_middle(x: usize, prefix_sum_one: &Vec<usize>) -> usize {
    if ((x as f32 + 1.).log2().ceil() as usize) < 1 {
        return 0;
    }
    let log2 = (x as f32 + 1.).log2().ceil() as usize - 1;
    let pow = 2_usize.pow(log2 as u32);
    let mut result = prefix_sum_one[log2];
    result += x - pow + 1;
    result += count_middle(x - pow, prefix_sum_one);
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let start = input.next().unwrap();
    let end = input.next().unwrap();

    let mut prefix_sum_one = vec![0_usize; 56];
    for idx in 1..56 {
        prefix_sum_one[idx] = 2 * prefix_sum_one[idx - 1] + 2_usize.pow((idx - 1) as u32);
    }

    println!(
        "{}",
        count_middle(end, &prefix_sum_one) - count_middle(start - 1, &prefix_sum_one)
    );
    Ok(())
}
