// XOR

use std::error::Error;
use std::fmt::{Debug, Display, Write};
use std::io::{stdin, Read};
use std::mem::size_of_val;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let n = input.next().unwrap() as usize;
    let data = input.by_ref().take(n).collect::<Vec<i64>>();
    let m = input.next().unwrap() as usize;

    let mut tree: LazySeg<Xor> = LazySeg::new(n, data);

    for _ in 0..m {
        if input.next().unwrap() == 1 {
            tree.modify(
                input.next().unwrap() as usize,
                input.next().unwrap() as usize + 1,
                input.next().unwrap(),
            );
        } else {
            let b = input.next().unwrap() as usize;
            writeln!(output, "{}", tree.query(b, b + 1))?;
        }
    }
    print!("{}", output);
    Ok(())
}

trait Spec {
    type S: Clone + Display + Debug + PartialEq;

    fn op(a: &Self::S, b: &Self::S) -> Self::S;
    fn identity() -> Self::S;
    fn combine(a: &Self::S, b: &Self::S) -> Self::S;
    fn default() -> Self::S;
}

enum Xor {}

impl Spec for Xor {
    type S = i64;
    fn op(&a: &Self::S, &b: &Self::S) -> Self::S {
        a ^ b
    }

    fn identity() -> Self::S {
        0
    }

    fn default() -> Self::S {
        0
    }

    fn combine(&a: &Self::S, &b: &Self::S) -> Self::S {
        a ^ b
    }
}

struct LazySeg<T: Spec> {
    val: Vec<T::S>,
    lazy: Vec<T::S>,
    n: usize,
    h: u32,
}

impl<T: Spec> LazySeg<T> {
    fn new(n: usize, data: impl IntoIterator<Item = T::S>) -> Self {
        let mut val = vec![T::default(); n];
        val.extend(data);
        let lazy = vec![T::default(); n];
        let h = size_of_val(&val[0]) as u32 * 8u32 - n.leading_zeros();
        for i in (1..n).rev() {
            val[i] = T::combine(&val[i << 1], &val[i << 1 | 1]);
        }
        Self { val, lazy, n, h }
    }

    fn apply(&mut self, idx: usize, len: usize, diff: &T::S) {
        for _ in 0..len {
            self.val[idx] = T::op(&self.val[idx], diff);
        }
        if idx < self.n {
            self.lazy[idx] = T::op(&self.lazy[idx], diff);
        }
    }

    fn update(&mut self, mut idx: usize) {
        let mut len = 2;
        while idx > 1 {
            idx >>= 1;
            self.val[idx] = T::combine(&self.val[idx << 1], &self.val[idx << 1 | 1]);
            if self.lazy[idx] != T::default() {
                for _ in 0..len {
                    self.val[idx] = T::op(&self.val[idx], &self.lazy[idx]);
                }
            }
            len <<= 1;
        }
    }

    fn propagate(&mut self, idx: usize) {
        let mut len = 1 << (self.h - 1);
        for shift in (1..=self.h).rev() {
            let ptr = idx >> shift;
            if self.lazy[ptr] != T::default() {
                let lazy_val = self.lazy[ptr].clone();
                self.apply(ptr << 1, len, &lazy_val);
                self.apply(ptr << 1 | 1, len, &lazy_val);
                self.lazy[ptr] = T::default();
            }
            len >>= 1;
        }
    }

    fn modify(&mut self, mut start: usize, mut end: usize, diff: T::S) {
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
                self.apply(p_start, len, &diff);
                p_start += 1;
            }
            if p_end & 1 == 1 {
                p_end -= 1;
                self.apply(p_end, len, &diff);
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

    fn query(&mut self, start: usize, end: usize) -> T::S {
        let mut p_start = start + self.n;
        let mut p_end = end + self.n;
        self.propagate(p_start);
        if start < end {
            self.propagate(p_end - 1);
        }
        let mut result = T::identity();
        while p_start < p_end {
            if p_start & 1 == 1 {
                result = T::combine(&result, &self.val[p_start]);
                p_start += 1;
            }
            if p_end & 1 == 1 {
                p_end -= 1;
                result = T::combine(&result, &self.val[p_end]);
            }
            p_start >>= 1;
            p_end >>= 1;
        }

        result
    }
}
