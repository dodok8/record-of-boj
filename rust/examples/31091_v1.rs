// 거짓말
use std::cmp::{max, min};
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

/// 만약 거짓말쟁이가 k 명 있다면,
/// - `x > k`, 즉  `x = k +1`이상이 거짓말을 하고 있다.
/// - `x < k` ,`x = k - 1` 이하가 거짓말을 하고 있다.
/// 이 두 경우가 거짓말이 된다.
/// 즉 [0,k] 까지의 경우에 대해 저걸 이용해서 계산하면 되는데, 누적합을 이용하면 반복횟수를 줄일 수 있을 것이다.
/// num_ge[k] = x > k 명 이상(ge)라는 사람의 누적합
/// num_le[k] = x < k 명 이하(le)라는 사람의 누적합
/// 거짓말 쟁이가 K + 1이상, N 이라고 하는 사람의 수는 num_ge[N] - num_ge[k]이다.
/// 거짓말 쟁이가 K 명 이하라고 하는 사람의 수는 num_le
/// num_le 와 num_ge 는 입력 받을 때 만들 수 있다.
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i64>);
    let n = input.next().unwrap() as usize;
    let mut num_ge = vec![0; n + 1];
    let mut num_le = vec![0; n + 1];

    for _ in 0..n {
        let k = input.next().unwrap();
        if k > 0 {
            let k = k as usize;
            for idx in k..(n + 1) {
                num_ge[idx] += 1;
            }
        } else {
            let k = k.abs() as usize;
            for idx in k..(n + 1) {
                num_le[idx] += 1;
            }
        }
    }
    println!("{:?}", num_ge);
    println!("{:?}", num_le);

    let mut results = Vec::new();
    for num_liar in 1..n {
        if num_liar == num_le[num_liar - 1] + num_ge[n] - num_ge[num_liar] {
            results.push(num_liar)
        }
    }
    writeln!(output, "{}", results.len()).unwrap();
    for num in results {
        write!(output, "{} ", num).unwrap();
    }
    println!("{}", output);
    Ok(())
}
