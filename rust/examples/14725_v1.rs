// 개미굴

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

struct Trie<'a> {
    tree: BTreeMap<&'a str, Trie<'a>>,
    is_word: bool,
}

impl<'a> Trie<'a> {
    fn insert(&mut self, word: &[&'a str]) {
        if word.is_empty() {
            return;
        }

        let entry = self.tree.entry(word[0]).or_insert_with(|| Trie {
            tree: BTreeMap::new(),
            is_word: word.len() == 1,
        });
        entry.insert(&word[1..]);
    }

    fn print(&self, depth: usize, output: &mut String) {
        for (key, next_tree) in &self.tree {
            for _ in 0..depth {
                write!(output, "--").unwrap();
            }
            writeln!(output, "{}", key).unwrap();
            next_tree.print(depth + 1, output);
        }
    }

    fn find(&self, word: &[&str], depth: usize) -> (usize, bool) {
        if word.is_empty() {
            return (depth, self.is_word);
        }

        if let Some(next_trie) = self.tree.get(word[0]) {
            next_trie.find(&word[1..], depth + 1)
        } else {
            (depth, false)
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<usize>()?;
    let mut trie = Trie {
        is_word: false,
        tree: BTreeMap::new(),
    };
    for _ in 0..n {
        let k = input.next().unwrap().parse::<usize>()?;
        let word: Vec<&str> = input.by_ref().take(k).collect();
        trie.insert(&word);
    }
    trie.print(0, &mut output);
    print!("{}", output);
    Ok(())
}
