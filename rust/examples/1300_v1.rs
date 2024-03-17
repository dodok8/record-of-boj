// K번째 수

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
    let k = input.next().unwrap();

    let mut start = 1_usize;
    let mut end = k;
    // B[k] = x 에서 x는 k 보다 클 수 없다.(k는 x이하인 수의 갯수를 의미하는데, i*j = j*i 이라 중복 생기고 그러니깐, 모조건 작아진다.)

    while start < end {
        let mid = (start + end) / 2;

        let mut cnt = 0;
        for idx in 1..=n {
            cnt += min(mid / idx, n);
        }

        if cnt >= k {
            end = mid;
        } else {
            start = mid + 1;
        }
    }
    writeln!(output, "{}", start).unwrap();
    print!("{}", output);
    Ok(())
}
