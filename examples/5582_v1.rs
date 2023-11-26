// 공통 부분 문자열
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

trait LongestCommonSubstring {
    fn get_longest_substring(&self, other: &str) -> Vec<String>;
}

impl LongestCommonSubstring for String {
    fn get_longest_substring(&self, other: &str) -> Vec<String> {
        let mut cache = vec![vec![0; other.len() + 1]; self.len() + 1];
        let mut result_idx: Vec<usize> = Vec::new();
        let mut length_counter = 0;
        for (self_idx, self_char) in self.chars().enumerate() {
            for (other_idx, other_char) in other.chars().enumerate() {
                if self_char == other_char {
                    cache[self_idx + 1][other_idx + 1] = cache[self_idx][other_idx] + 1;
                    if cache[self_idx + 1][other_idx + 1] > length_counter {
                        length_counter = cache[self_idx + 1][other_idx + 1];
                        result_idx.clear();
                    }
                    if cache[self_idx + 1][other_idx + 1] == length_counter {
                        result_idx.push(self_idx);
                    }
                } else {
                    cache[self_idx + 1][other_idx + 1] = 0;
                }
            }
        }
        let mut result: Vec<String> = Vec::new();
        for idx in result_idx {
            result.push(self[(idx - length_counter + 1)..(idx + 1)].to_string())
        }
        result
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let first_str = input.next().unwrap().to_string();
    let second_str = input.next().unwrap().to_string();
    let lcs_vec = first_str.get_longest_substring(&second_str);
    if lcs_vec.is_empty() {
        writeln!(output, "0").unwrap();
    } else {
        writeln!(output, "{}", lcs_vec[0].len()).unwrap();
    }
    print!("{}", output);
    Ok(())
}
