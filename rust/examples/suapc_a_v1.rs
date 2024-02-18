// 가상 검증 기술

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let count_test = input.next().unwrap();
    for _ in 0..count_test {
        let t_a = input.next().unwrap();
        let t_b = input.next().unwrap();
        let v_a = input.next().unwrap();
        let v_b = input.next().unwrap();

        let end_b = t_b * v_b;

        if end_b >= t_a * v_a {
            writeln!(output, "{}", end_b).unwrap();
            continue;
        }
        let d = [end_b % t_a, t_a - end_b % t_a];
        let mut curr_step = 0;
        let mut curr_a = end_b / t_a;
        let mut curr_t = end_b;
        while curr_a != v_a {
            curr_t += d[curr_step % 2];
            curr_a = curr_t / t_a + (curr_t - end_b) / t_a;
            curr_step += 1;
        }
        writeln!(output, "{}", curr_t).unwrap();
    }
    print!("{}", output);
    Ok(())
}
