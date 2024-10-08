// 전설

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

struct Trie {
    tree: HashMap<u8, Trie>,
    is_word: bool,
}

impl Trie {
    fn insert(&mut self, word: &[u8]) {
        if word.is_empty() {
            return;
        }
        let entry = self.tree.entry(word[0]).or_insert_with(|| Trie {
            tree: HashMap::new(),
            is_word: false,
        });
        if word.len() == 1 {
            entry.is_word = true;
        }
        entry.insert(&word[1..]);
    }

    fn find(&self, word: &[u8], idx: usize, result: &mut Vec<usize>) {
        if self.is_word {
            // 단어라서 넣음
            result.push(idx - 1);
        }

        if word.is_empty() {
            return;
        }

        if let Some(next_trie) = self.tree.get(&word[0]) {
            next_trie.find(&word[1..], idx + 1, result);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let num_c = input.next().unwrap().parse::<usize>()?;
    let num_n = input.next().unwrap().parse::<usize>()?;

    let mut color_trie = Trie {
        is_word: false,
        tree: HashMap::new(),
    };

    for color in input.by_ref().take(num_c).map(|x| x.as_bytes()) {
        color_trie.insert(color);
    }

    let nicknames: Vec<&[u8]> = input.by_ref().take(num_n).map(|x| x.as_bytes()).collect();

    let nickname_set: HashSet<&[u8]> = nicknames.into_iter().collect();

    let num_q = input.next().unwrap().parse::<usize>()?;

    for _ in 0..num_q {
        let team = input.next().unwrap().as_bytes();

        let mut color_result = vec![];

        color_trie.find(team, 0, &mut color_result);
        let mut result = false;

        for cdx in color_result {
            if nickname_set.contains(&team[(cdx + 1)..]) {
                result = true;
                break;
            }
        }
        if !result {
            writeln!(output, "NO")?;
        } else {
            writeln!(output, "YES")?;
        }
    }
    print!("{}", output);
    Ok(())
}
