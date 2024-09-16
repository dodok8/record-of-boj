// XOR

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

struct Tree {
    val: Vec<usize>,
    n: usize,
}

impl Tree {
    fn new(n: usize, data: impl IntoIterator<Item = usize>) -> Self {
        let mut val = vec![0; n];
        val.extend(data);
        for i in (1..n).rev() {
            val[i] = val[i << 1] ^ val[i << 1 | 1]
        }
        Self { val, n }
    }
    fn modify(&mut self, start: usize, end: usize, diff: usize) {
        let mut start = start + self.n;
        let mut end = end + self.n;
        while start < end {
            if start & 1 == 1 {
                self.val[start] ^= diff;
                start += 1;
            }
            if end & 1 == 1 {
                end -= 1;
                self.val[end] ^= diff;
            }
            start >>= 1;
            end >>= 1;
        }
    }

    fn query(&self, idx: usize) -> usize {
        let mut result = 0;
        let mut idx = idx + self.n;
        while idx > 0 {
            result ^= self.val[idx];
            idx >>= 1
        }
        result
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let data = input.by_ref().take(n).collect::<Vec<usize>>();
    let m = input.next().unwrap();

    let mut tree = Tree::new(n, data);
    for _ in 0..m {
        let t = input.next().unwrap();
        if t == 1 {
            let a = input.next().unwrap();
            let b = input.next().unwrap();
            let c = input.next().unwrap();
            tree.modify(a, b + 1, c);
        } else {
            let a = input.next().unwrap();
            writeln!(output, "{}", tree.query(a))?;
        }
    }
    print!("{}", output);
    Ok(())
}
