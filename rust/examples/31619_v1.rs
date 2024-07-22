// 買い物 2 (Shopping 2)

trait ArqSpec {
    type S: Clone;

    fn op(a: &Self::S, b: &Self::S) -> Self::S;
    fn identity() -> Self::S;
    fn apply(a: &Self::S, b: &Self::S) -> Self::S;
}

enum AssignDiscount {}

impl ArqSpec for AssignDiscount {
    type S = usize;
    // 쿼리 연산
    fn op(&a: &Self::S, &b: &Self::S) -> Self::S {
        a + b
    }

    fn identity() -> Self::S {
        0
    }

    // 업뎃 연산
    fn apply(&a: &Self::S, &b: &Self::S) -> Self::S {
        b
    }
}

struct StaticArq<T: ArqSpec> {
    val: Vec<T::S>,
    n: usize,
}

impl<T: ArqSpec> StaticArq<T> {
    /// Initializes a static balanced binary tree on top of the given sequence.
    fn new(data: &[T::S]) -> Self {
        let n = data.len();
        let h = n.next_power_of_two().trailing_zeros() as usize;
        let size = 2usize.pow(h as u32 + 1);
        let mut val = vec![T::identity(); size];

        fn fill<T: ArqSpec>(
            idx: usize,
            start: usize,
            end: usize,
            val: &mut [T::S],
            data: &[T::S],
        ) -> T::S {
            if start == end {
                val[idx] = data[start].clone();
                return val[idx].clone();
            }

            let mid = (start + end) / 2;
            let left_result = fill::<T>(idx * 2 + 1, start, mid, val, data);
            let right_result = fill::<T>(idx * 2 + 2, mid + 1, end, val, data);
            val[idx] = T::op(&left_result, &right_result);
            val[idx].clone()
        }

        fill::<T>(0, 0, n - 1, &mut val, data);
        // 이 문제에서는 빈 상태에서 출발하므로, 갱신이 필요가 없음.

        Self { val, n }
    }

    fn update(&mut self, idx: usize, new_value: T::S) {
        let end = self.n - 1;
        let start = 0usize;

        fn recursive_update<T: ArqSpec>(
            node_idx: usize,
            start: usize,
            end: usize,
            new_value: T::S,
            tree: &mut StaticArq<T>,
            idx: usize,
        ) {
            if start == end {
                let result = T::apply(&tree.val[node_idx], &new_value);
                tree.val[node_idx] = result;
            } else {
                let mid = (start + end) / 2;

                if start <= idx && idx <= mid {
                    recursive_update(node_idx * 2 + 1, start, mid, new_value, tree, idx);
                } else {
                    recursive_update(node_idx * 2 + 2, mid + 1, end, new_value, tree, idx);
                }
                let result = T::op(&tree.val[node_idx * 2 + 1], &tree.val[node_idx * 2 + 2]);
                tree.val[node_idx] = result;
            }
        }

        recursive_update(0, start, end, new_value, self, idx);
    }

    fn query(&self, start: usize, end: usize) -> T::S {
        fn recursive_query<T: ArqSpec>(
            node_idx: usize,
            node_start: usize,
            node_end: usize,
            tree: &StaticArq<T>,
            start: usize,
            end: usize,
        ) -> T::S {
            if end < node_start || node_end < start {
                T::identity()
            } else if start <= node_start && node_end <= end {
                return tree.val[node_idx].clone();
            } else {
                let mid = (node_start + node_end) / 2;
                return T::op(
                    &recursive_query(node_idx * 2 + 1, node_start, mid, tree, start, end),
                    &recursive_query(node_idx * 2 + 2, mid + 1, node_end, tree, start, end),
                );
            }
        }

        recursive_query(0, 0, self.n - 1, self, start, end)
    }
}

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let m = input.next().unwrap();
    let q = input.next().unwrap();

    let mut categories = vec![Vec::new(); m];
    let mut data = Vec::new();

    for idx in 0..n {
        let cost = input.next().unwrap();
        let category = input.next().unwrap() - 1;
        categories[category].push(idx);
        data.push(cost);
    }

    let mut trees: Vec<StaticArq<AssignDiscount>> = Vec::new();

    for day in 0..q {
        let mut new_data = data.clone();
        for &idx in &categories[day] {
            new_data[idx] = data[idx] / 2;
        }
        trees.push(StaticArq::new(&new_data))
    }

    for _ in 0..q {
        let day = input.next().unwrap() - 1;
        let start = input.next().unwrap() - 1;
        let end = input.next().unwrap() - 1;

        writeln!(output, "{}", trees[day].query(start, end)).unwrap();
    }
    print!("{}", output);
    Ok(())
}
