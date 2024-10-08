// 전설

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

struct Trie {
    tree: BTreeMap<u8, Trie>,
    is_word: bool,
}

impl Trie {
    fn insert(&mut self, word: &[u8]) {
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

    fn find(&self, word: &[u8], idx: usize, result: &mut Vec<bool>) {
        if self.is_word {
            // 단어라서 넣음
            result[idx - 1] = true;
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
        tree: BTreeMap::new(),
    };

    let mut nickname_trie = Trie {
        is_word: false,
        tree: BTreeMap::new(),
    };

    for color in input
        .by_ref()
        .take(num_c)
        .map(|x| x.chars().map(|c| c as u8).collect::<Vec<u8>>())
    {
        color_trie.insert(&color);
    }

    for nickname in input
        .by_ref()
        .take(num_n)
        .map(|x| x.chars().map(|c| c as u8).collect::<Vec<u8>>())
    {
        nickname_trie.insert(&nickname);
    }

    let num_q = input.next().unwrap().parse::<usize>()?;

    for _ in 0..num_q {
        let mut team = input
            .next()
            .unwrap()
            .chars()
            .map(|c| c as u8)
            .collect::<Vec<u8>>();

        let mut nickname_result = vec![false; team.len()];
        let mut color_result = vec![false; team.len()];

        color_trie.find(&team, 0, &mut color_result);
        team.reverse();
        nickname_trie.find(&team, 0, &mut nickname_result);
        // writeln!(output, "nick: {:?}", nickname_result)?;
        // writeln!(output, "color: {:?}", color_result)?;
        let mut result = false;
        for cdx in 0..=(team.len() - 2) {
            let ndx = team.len() - 2 - cdx;

            if nickname_result[ndx] && color_result[cdx] {
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
