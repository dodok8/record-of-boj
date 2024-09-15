// 구간 합 구하기 2

trait ArqSpec {
    type S: Clone + PartialEq + PartialOrd + Display + Debug;

    fn op(a: &Self::S, b: &Self::S) -> Self::S;
    fn identity() -> Self::S;
    fn apply(a: &Self::S, b: &Self::S) -> Self::S;
}

enum AssignSum {}

impl ArqSpec for AssignSum {
    type S = i64;
    fn op(&a: &Self::S, &b: &Self::S) -> Self::S {
        a + b
    }

    fn identity() -> Self::S {
        0
    }

    fn apply(&a: &Self::S, &b: &Self::S) -> Self::S {
        a + b
    }
}

struct StaticArq<T: ArqSpec> {
    val: Vec<(T::S, T::S)>,
    n: usize,
}

impl<T: ArqSpec> StaticArq<T> {
    /// Initializes a static balanced binary tree on top of the given sequence.
    pub fn new(data: &[T::S]) -> Self {
        let n = data.len();
        let h = n.next_power_of_two().trailing_zeros() as usize;
        let size = 2usize.pow(h as u32 + 1);

        let mut extended_data = vec![T::identity(); n.next_power_of_two()];
        extended_data[..n].clone_from_slice(data);
        let data = extended_data;

        let mut val = vec![(T::identity(), T::identity()); size - 1];

        fn fill<T: ArqSpec>(
            idx: usize,
            start: usize,
            end: usize,
            val: &mut [(T::S, T::S)],
            data: &[T::S],
        ) -> T::S {
            if start == end {
                val[idx].0 = data[start].clone();
                return val[idx].0.clone();
            }

            let mid = (start + end) / 2;
            let left_result = fill::<T>(idx * 2 + 1, start, mid, val, data);
            let right_result = fill::<T>(idx * 2 + 2, mid + 1, end, val, data);
            val[idx].0 = T::op(&left_result, &right_result);
            val[idx].0.clone()
        }

        fill::<T>(0, 0, data.len() - 1, &mut val, &data);

        Self { val, n: data.len() }
    }

    fn propagate(&mut self, idx: usize) {
        let h = ((idx + 1).ilog2() + 1) as usize;
        let w = self.n / 2usize.pow(h as u32 - 1);
        if self.val[idx].1 != T::identity() {
            for _ in 0..w {
                self.val[idx].0 = T::op(&self.val[idx].0, &self.val[idx].1);
            }
            if w != 1 {
                self.val[idx * 2 + 1].1 = T::op(&self.val[idx * 2 + 1].1, &self.val[idx].1);
                self.val[idx * 2 + 2].1 = T::op(&self.val[idx * 2 + 2].1, &self.val[idx].1);
            }
            self.val[idx].1 = T::identity();
        }
    }

    pub fn update(&mut self, start: usize, end: usize, new_value: T::S) {
        fn recursive_update<T: ArqSpec>(
            idx: usize,
            start: usize,
            end: usize,
            new_value: &T::S,
            tree: &mut StaticArq<T>,
        ) {
            let h = ((idx + 1).ilog2() + 1) as usize;
            let w = tree.n / 2usize.pow(h as u32 - 1);
            let d = 2usize.pow((h - 1) as u32) - 1;

            if w == 0 {
                return;
            }

            let curr_start = (idx - d) * w; // 현재 구간의 시작
            let curr_end = (idx - d + 1) * w - 1; // 현재 구간의 끝

            if curr_end < start || end < curr_start {
                return;
            }
            if curr_start >= start && curr_end <= end {
                tree.val[idx].1 = T::op(&tree.val[idx].1, new_value);
                tree.propagate(idx);
                return;
            }

            recursive_update(idx * 2 + 1, start, end, new_value, tree);
            recursive_update(idx * 2 + 2, start, end, new_value, tree);
            tree.val[idx].0 = T::apply(&tree.val[idx * 2 + 1].0, &tree.val[idx * 2 + 2].0);
        }

        recursive_update(0, start, end, &new_value, self);
    }

    pub fn query(&mut self, start: usize, end: usize) -> T::S {
        fn recursive_query<T: ArqSpec>(
            idx: usize,
            tree: &mut StaticArq<T>,
            start: usize,
            end: usize,
        ) -> T::S {
            let h = ((idx + 1).ilog2() + 1) as usize;
            let w = tree.n / 2usize.pow(h as u32 - 1);
            let d = 2usize.pow((h - 1) as u32) - 1;

            if w == 0 {
                return T::identity();
            }

            let curr_start = (idx - d) * w;
            let curr_end = (idx - d + 1) * w - 1;

            tree.propagate(idx);

            if start <= curr_start && curr_end <= end {
                return tree.val[idx].0.clone();
            }

            let left = recursive_query(idx * 2 + 1, tree, start, end);
            let right = recursive_query(idx * 2 + 2, tree, start, end);
            T::apply(&left, &right)
        }

        recursive_query(0, self, start, end)
    }
}

use std::error::Error;
use std::fmt::{Debug, Display, Write};
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let n = input.next().unwrap() as usize;
    let m = input.next().unwrap() as usize;
    let k = input.next().unwrap() as usize;

    let data = input.by_ref().take(n).collect::<Vec<i64>>();
    let mut tree: StaticArq<AssignSum> = StaticArq::new(&data);
    writeln!(output, "{:?}", tree.val)?;

    for _ in 0..(m + k) {
        if input.next().unwrap() == 1 {
            writeln!(output, "업데이트 쿼리")?;
            tree.update(
                input.next().unwrap() as usize - 1,
                input.next().unwrap() as usize - 1,
                input.next().unwrap(),
            );
            writeln!(output, "{:?}", tree.val)?;
        } else {
            writeln!(output, "출력 쿼리")?;
            writeln!(
                output,
                "{}",
                tree.query(
                    input.next().unwrap() as usize - 1,
                    input.next().unwrap() as usize - 1
                )
            )?;
            writeln!(output, "{:?}", tree.val)?;
        }
    }
    print!("{}", output);
    Ok(())
}
