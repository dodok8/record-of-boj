// AC

use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

struct Ac {
    reversed: bool,
    deque: VecDeque<usize>,
}

impl Ac {
    fn reverse(&mut self) {
        self.reversed = !self.reversed;
    }
    fn drop(&mut self) -> Option<usize> {
        if self.reversed {
            self.deque.pop_back()
        } else {
            self.deque.pop_front()
        }
    }
    fn get_result(&self) -> String {
        let mut result = String::new();
        result.push('[');
        if self.reversed {
            for &item in self.deque.iter().rev() {
                result.push_str(&item.to_string());
                result.push(',');
            }
        } else {
            for &item in self.deque.iter() {
                result.push_str(&item.to_string());
                result.push(',');
            }
        }
        result.pop(); // 마지막 쉼표 제거
        result.push(']');
        result
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    for _ in 0..input.next().unwrap().parse::<usize>().unwrap() {
        let commands = input.next().unwrap();
        let _len = input.next().unwrap().parse::<usize>().unwrap();
        let s = input.next().unwrap().trim_matches('[').trim_matches(']');
        let vec: Vec<usize> = s
            .split(',')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let mut curr_ac = Ac {
            reversed: false,
            deque: VecDeque::from(vec),
        };
        let mut is_error = false;
        for command in commands.chars() {
            match command {
                'R' => {
                    curr_ac.reverse();
                }
                'D' => {
                    if curr_ac.drop().is_none() {
                        is_error = true;
                        break;
                    }
                }
                _ => break,
            }
        }
        if is_error {
            writeln!(output, "error").unwrap();
        } else {
            writeln!(output, "{}", curr_ac.get_result()).unwrap();
        }
    }
    print!("{}", output);
    Ok(())
}
