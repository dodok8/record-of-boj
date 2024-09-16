use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let mut data = input.by_ref().take(n).collect::<Vec<usize>>();
    let m = input.next().unwrap();

    let mut diff = vec![0; n + 1];

    fn update(diff: &mut Vec<usize>, start: usize, end: usize, val: usize) {
        diff[start] ^= val;
        if end + 1 < diff.len() {
            diff[end + 1] ^= val;
        }
    }

    fn query(diff: &Vec<usize>, idx: usize) -> usize {
        let mut result = 0;
        for jdx in 0..=idx {
            result ^= diff[jdx];
        }
        result
    }

    for idx in 0..n {
        update(&mut diff, idx, idx, data[idx]);
    }
    for _ in 0..m {
        let t = input.next().unwrap();
        if t == 1 {
            let a = input.next().unwrap();
            let b = input.next().unwrap();
            let c = input.next().unwrap();
            update(&mut diff, a, b, c);
        } else {
            let a = input.next().unwrap();
            let ans = query(&diff, a);
            writeln!(output, "{}", ans)?;
        }
    }
    print!("{}", output);
    Ok(())
}
