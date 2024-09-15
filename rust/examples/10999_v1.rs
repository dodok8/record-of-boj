// Incorrect code

type T = i64;
struct Arq {
    val: Vec<i64>,
    lazy: Vec<i64>,
    n: usize,
    h: u32,
}

impl Arq {
    fn combine(l: T, r: T) -> T {
        l + r
    }

    fn new(n: usize, data: impl IntoIterator<Item = i64>) -> Self {
        let mut val = vec![T::default(); n];
        val.extend(data);
        let lazy = vec![T::default(); n];
        let h = 64 - n.leading_zeros();
        for idx in (1..n).rev() {
            val[idx] = Self::combine(val[idx << 1], val[idx << 1 | 1]);
        }
        Self { val, lazy, n, h }
    }

    fn apply(&mut self, idx: usize, len: usize, amount: T) {
        self.val[idx] += len as T * amount;
        if idx < self.n {
            self.lazy[idx] += amount;
        }
    }

    fn update(&mut self, mut idx: usize) {
        let mut len = 2;
        while idx > 1 {
            idx >>= 1;
            self.val[idx] = Self::combine(self.val[idx << 1], self.val[idx << 1 | 1]);
            if self.lazy[idx] != T::default() {
                self.val[idx] += len as T * self.lazy[idx];
            }
            len <<= 1;
        }
    }

    fn propagate(&mut self, idx: usize) {
        let mut len = 1 << (self.h - 1);
        for shift in (1..=self.h).rev() {
            let ptr = idx >> shift;
            if self.lazy[ptr] != T::default() {
                self.apply(ptr << 1, len, self.lazy[ptr]);
                self.apply(ptr << 1 | 1, len, self.lazy[ptr]);
                self.lazy[ptr] = 0;
            }
            len >>= 1;
        }
    }

    fn modify(&mut self, mut start: usize, mut end: usize, diff: T) {
        start += self.n;
        end += self.n;

        let mut p_start = start;
        let mut p_end = end;
        self.propagate(start);
        if start < end {
            self.propagate(end - 1);
        }
        let mut len = 1;

        while p_start < p_end {
            if p_start & 1 == 1 {
                self.apply(p_start, len, diff);
                p_start += 1;
            }
            if p_end & 1 == 1 {
                p_end -= 1;
                self.apply(p_end, len, diff);
            }
            p_end >>= 1;
            p_start >>= 1;
            len <<= 1;
        }
        self.update(start);
        if start < end {
            self.update(end - 1);
        }
    }

    fn query(&mut self, start: usize, end: usize) -> T {
        let mut p_start = start + self.n;
        let mut p_end = end + self.n;
        self.propagate(p_start);
        if start < end {
            self.propagate(p_end - 1);
        }
        let mut result = T::default();
        while p_start < p_end {
            if p_start & 1 == 1 {
                result = Self::combine(result, self.val[p_start]);
                p_start += 1;
            }
            if p_end & 1 == 1 {
                p_end -= 1;
                result = Self::combine(self.val[p_end], result);
            }
            p_start >>= 1;
            p_end >>= 1;
        }

        result
    }
}

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let n = input.next().unwrap() as usize;
    let m = input.next().unwrap() as usize;
    let k = input.next().unwrap() as usize;

    let data = input.by_ref().take(n);
    let mut tree = Arq::new(n, data);

    for _ in 0..(m + k) {
        if input.by_ref().next().unwrap() == 1 {
            tree.modify(
                input.by_ref().next().unwrap() as usize - 1,
                input.by_ref().next().unwrap() as usize,
                input.by_ref().next().unwrap(),
            );
        } else {
            writeln!(
                output,
                "{}",
                tree.query(
                    input.next().unwrap() as usize - 1,
                    input.next().unwrap() as usize
                )
            )?;
        }
    }
    print!("{}", output);
    Ok(())
}
