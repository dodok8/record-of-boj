// Secret Message

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

struct Trie {
    tree: BTreeMap<usize, Trie>,
    word: usize,
    count: usize,
}

impl Trie {
    fn insert(&mut self, word: &[usize]) {
        if word.is_empty() {
            return;
        }
        let entry = if let Some(trie) = self.tree.get_mut(&word[0]) {
            trie.count += 1;
            trie
        } else {
            self.tree.insert(
                word[0],
                Trie {
                    tree: BTreeMap::new(),
                    word: 0,
                    count: 0,
                },
            );
            self.tree.get_mut(&word[0]).unwrap()
        };

        if word.len() == 1 {
            entry.word += 1;
        }
        entry.insert(&word[1..]);
    }

    fn dfs(&self, word: &[usize]) -> usize {
        let mut result = 0;

        fn recursive(t: &Trie, result: &mut usize, word: &[usize]) {
            if t.word != 0 {
                *result += t.word;
            }

            if word.is_empty() {
                for v in t.tree.values() {
                    recursive(v, result, word);
                }
                if t.tree.is_empty() {
                    *result += t.count;
                }
            } else if let Some(next_trie) = t.tree.get(&word[0]) {
                recursive(next_trie, result, &word[1..]);
            }
        }
        recursive(self, &mut result, word);
        result
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let m = input.next().unwrap();
    let n = input.next().unwrap();

    let mut zero_one_trie = Trie {
        word: 0,
        count: 0,
        tree: BTreeMap::new(),
    };
    for _ in 0..m {
        let l = input.next().unwrap();
        let word = input.by_ref().take(l).collect::<Vec<usize>>();
        zero_one_trie.insert(&word);
    }

    for _ in 0..n {
        let l = input.next().unwrap();
        let word = input.by_ref().take(l).collect::<Vec<usize>>();

        writeln!(output, "{}", zero_one_trie.dfs(&word))?;
    }

    print!("{}", output);
    Ok(())
}
