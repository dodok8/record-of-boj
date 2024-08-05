// 링고와 순열

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let mut k = input.next().unwrap();

    let mut lower_bound = 0;
    let mut upper_bound = n - 1;

    let mut nums = vec![0; n];
    for idx in 0..n {
        let num_rev = n - idx - 1;
        if k >= num_rev {
            k -= n - 1 - idx;
            nums[idx] = upper_bound;
            upper_bound -= 1;
        } else {
            nums[idx] = lower_bound;
            lower_bound += 1; // 이 부분 때문에 정상 종료 되었다면, lower_bound가 upper_bound 보다 1이 더 크다. 만약 그렇지 않다면, 아직 다 놓지 못한 숫자가 있다는 뜻이다.
        }
    }

    if k != 0 || upper_bound >= lower_bound {
        write!(output, "-1")?;
    }
    for num in nums {
        write!(output, "{} ", num + 1)?;
    }
    println!("{}", output);
    Ok(())
}
