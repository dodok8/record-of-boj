// LCS
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

trait LongestCommonSubsequence {
    fn get_len_lcs(&self, other: &str) -> u64;
}

impl LongestCommonSubsequence for String {
    fn get_len_lcs(&self, other: &str) -> u64 {
        let mut cache = vec![vec![0_u64; other.len() + 1]; self.len() + 1];
        let mut longest_idx: (usize, usize) = (0, 0);
        for (self_idx, self_char) in self.chars().enumerate() {
            for (other_idx, other_char) in other.chars().enumerate() {
                if self_char == other_char {
                    cache[self_idx + 1][other_idx + 1] = cache[self_idx][other_idx] + 1;
                    if cache[self_idx + 1][other_idx + 1] > cache[longest_idx.0][longest_idx.1] {
                        longest_idx = (self_idx + 1, other_idx + 1);
                    }
                } else {
                    cache[self_idx + 1][other_idx + 1] = *[
                        cache[self_idx][other_idx + 1],
                        cache[self_idx + 1][other_idx],
                    ]
                    .iter()
                    .max()
                    .unwrap();
                }
            }
        }
        cache[longest_idx.0][longest_idx.1]
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let first_str = input.next().unwrap().to_string();
    let second_str = input.next().unwrap().to_string();
    writeln!(output, "{}", first_str.get_len_lcs(&second_str)).unwrap();
    print!("{}", output);
    Ok(())
}
