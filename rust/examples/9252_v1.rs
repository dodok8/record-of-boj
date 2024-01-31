// LCS 2
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

trait LongestCommonSubsequence {
    fn get_lcs(&self, other: &str) -> (usize, String);
}

impl LongestCommonSubsequence for String {
    fn get_lcs(&self, other: &str) -> (usize, String) {
        let mut cache = vec![vec![(0, (0, 0), false); other.len() + 1]; self.len() + 1];
        let mut longest_idx: (usize, usize) = (0, 0);
        let mut curr_len = 0;
        for (self_idx, self_char) in (1..).zip(self.bytes()) {
            for (other_idx, other_char) in (1..).zip(other.bytes()) {
                if self_char == other_char {
                    cache[self_idx][other_idx].0 = cache[self_idx - 1][other_idx - 1].0 + 1;
                    if cache[self_idx][other_idx].0 == 1 {
                        cache[self_idx][other_idx].1 = (self_idx, other_idx);
                    } else {
                        cache[self_idx][other_idx].1 = (self_idx - 1, other_idx - 1);
                    }
                    cache[self_idx][other_idx].2 = true;

                    if cache[self_idx][other_idx].0 > curr_len {
                        longest_idx = (self_idx, other_idx);
                        curr_len = cache[self_idx][other_idx].0;
                    }
                } else if cache[self_idx - 1][other_idx].0 > cache[self_idx][other_idx - 1].0 {
                    cache[self_idx][other_idx].0 = cache[self_idx - 1][other_idx].0;
                    cache[self_idx][other_idx].1 = if cache[self_idx - 1][other_idx].2 {
                        (self_idx - 1, other_idx)
                    } else {
                        cache[self_idx - 1][other_idx].1
                    };
                } else {
                    cache[self_idx][other_idx].0 = cache[self_idx][other_idx - 1].0;
                    cache[self_idx][other_idx].1 = if cache[self_idx][other_idx - 1].2 {
                        (self_idx, other_idx - 1)
                    } else {
                        cache[self_idx][other_idx - 1].1
                    };
                }
            }
        }

        let mut result = Vec::new();

        loop {
            let next = cache[longest_idx.0][longest_idx.1].1;
            if cache[longest_idx.0][longest_idx.1].2 {
                result.push(self.chars().nth(longest_idx.0 - 1).unwrap())
            }
            if longest_idx == next {
                break (curr_len, result.iter().rev().collect());
            } else {
                longest_idx = next;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let first_str = input.next().unwrap().to_string();
    let second_str = input.next().unwrap().to_string();
    let (len, str) = first_str.get_lcs(&second_str);
    writeln!(output, "{}", len).unwrap();
    writeln!(output, "{}", str).unwrap();
    print!("{}", output);
    Ok(())
}
