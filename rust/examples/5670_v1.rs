// 휴대폰 지판

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

struct Trie {
    tree: BTreeMap<char, Trie>,
    is_word: bool,
}

impl Trie {
    fn insert(&mut self, word: &[char]) {
        if word.is_empty() {
            return;
        }
        let entry = self.tree.entry(word[0]).or_insert_with(|| Trie {
            tree: BTreeMap::new(),
            is_word: false,
        });
        if word.len() == 1 {
            entry.is_word = true;
        }
        entry.insert(&word[1..]);
    }

    fn print(&self, depth: usize, output: &mut String) {
        for (key, next_trie) in &self.tree {
            for _ in 0..depth {
                write!(output, "--").unwrap();
            }
            writeln!(output, "{} / {}", key, next_trie.is_word).unwrap();
            next_trie.print(depth + 1, output);
        }
    }

    fn find(&self, word: &[char], press: usize) -> Result<usize, Box<dyn Error>> {
        if word.is_empty() {
            return Ok(press);
        }

        if let Some(next_trie) = self.tree.get(&word[0]) {
            if self.is_word {
                // 단어라서 증가
                next_trie.find(&word[1..], press + 1)
            } else if self.tree.len() > 1 {
                // 분기점이라서 증가
                next_trie.find(&word[1..], press + 1)
            } else if press == 0 {
                // 첫 글자라서 증가
                next_trie.find(&word[1..], press + 1)
            } else {
                next_trie.find(&word[1..], press)
            }
        } else {
            Err("Not found".to_string().into())
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    while let Some(n) = input.next() {
        let mut trie = Trie {
            is_word: false,
            tree: BTreeMap::new(),
        };
        let n = n.parse::<usize>()?;
        let words: Vec<Vec<char>> = input
            .by_ref()
            .take(n)
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect();
        for word in words.iter() {
            trie.insert(word);
        }
        let ans = words
            .iter()
            .map(|word| trie.find(word, 0).unwrap())
            .reduce(|a, b| a + b)
            .unwrap() as f64
            / words.len() as f64;
        writeln!(output, "{:.2}", ans)?;
    }

    print!("{}", output);
    Ok(())
}
