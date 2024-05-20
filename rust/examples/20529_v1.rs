// 가장 가까운 세 사람의 심리적 거리

fn get_combinations(vec: Vec<usize>, n: usize) -> Vec<Vec<usize>> {
    fn combinations_helper(
        vec: &[usize],
        n: usize,
        prefix: &mut Vec<usize>,
        result: &mut Vec<Vec<usize>>,
    ) {
        if n == 0 {
            result.push(prefix.clone());
            return;
        }

        for (i, &item) in vec.iter().enumerate() {
            let remaining = vec[i + 1..].to_vec();
            prefix.push(item);
            combinations_helper(&remaining, n - 1, prefix, result);
            prefix.pop();
        }
    }

    let mut result = Vec::new();
    let mut prefix = Vec::new();
    combinations_helper(&vec, n, &mut prefix, &mut result);
    result
}

fn mbti_to_usize(s: &str) -> usize {
    let mut result = 0;
    for (idx, c) in s.chars().enumerate() {
        let sum: usize = match c {
            'E' | 'S' | 'T' | 'J' => 0,
            _ => 1,
        } * 2_usize.pow(idx as u32);
        result += sum;
    }
    result
}

fn count_different_bits(a: usize, b: usize) -> usize {
    (a ^ b).count_ones() as usize
}

use std::cmp;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let mut dp = vec![vec![0; 16]; 16];

    for idx in 0..16 {
        for jdx in 0..16 {
            dp[idx][jdx] = count_different_bits(idx, jdx);
            dp[jdx][idx] = dp[idx][jdx];
        }
    }
    for _ in 0..input.next().unwrap().parse::<usize>().unwrap() {
        let n = input.next().unwrap().parse::<usize>().unwrap();
        let mut result = usize::MAX;
        if n > 32 {
            let _mbtis = input
                .by_ref()
                .take(n)
                .map(mbti_to_usize)
                .collect::<Vec<usize>>();
            result = 0;
        } else {
            let mbtis = input
                .by_ref()
                .take(n)
                .map(mbti_to_usize)
                .collect::<Vec<usize>>();

            for c in get_combinations(mbtis, 3) {
                let temp = dp[c[0]][c[1]] + dp[c[1]][c[2]] + dp[c[2]][c[0]];
                result = cmp::min(result, temp);
                if temp == 0 {
                    break;
                }
            }
        }
        writeln!(output, "{}", result).unwrap();
    }
    print!("{}", output);
    Ok(())
}
