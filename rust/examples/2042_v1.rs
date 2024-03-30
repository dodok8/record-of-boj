// 구간 합 구하기
// 세그먼트 트리의 제일 기본 형태

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};
use std::ops::{Add, AddAssign};

struct SegmentTree<T: Copy + Add> {
    nodes: Vec<T>,
    n: usize,
}

impl<T> SegmentTree<T>
where
    T: Copy + Add<Output = T> + AddAssign + Default,
{
    fn from(data: &Vec<T>) -> Self {
        let n = data.len();
        let h = n.next_power_of_two().trailing_zeros() as usize;
        let mut nodes: Vec<Option<T>> = vec![None; 2_usize.pow(h as u32 + 1)];

        fn fill<T: Copy + Add<Output = T>>(
            idx: usize,
            start: usize,
            end: usize,
            nodes: &mut Vec<Option<T>>,
            data: &Vec<T>,
        ) -> std::option::Option<T> {
            if start == end {
                nodes[idx] = Some(data[start]);
                return nodes[idx];
            }

            let mid = (start + end) / 2;
            nodes[idx] = Some(
                fill(idx * 2 + 1, start, mid, nodes, data).unwrap()
                    + fill(idx * 2 + 2, mid + 1, end, nodes, data).unwrap(),
            );
            nodes[idx]
        }

        fill(0, 0, data.len() - 1, &mut nodes, data);

        let nodes: Vec<T> = nodes.into_iter().flatten().collect();

        SegmentTree {
            nodes,
            n: data.len(),
        }
    }
    fn update(&mut self, idx: usize, new_value: T) {
        let end = self.n - 1;
        let start = 0_usize;
        fn recursive_update<T: Copy + Add<Output = T>>(
            node_idx: usize,
            start: usize,
            end: usize,
            new_value: T,
            tree: &mut SegmentTree<T>,
            idx: usize,
        ) {
            if start == end {
                tree.nodes[node_idx] = new_value;
            } else {
                let mid = (start + end) / 2;

                if start <= idx && idx <= mid {
                    recursive_update(node_idx * 2 + 1, start, mid, new_value, tree, idx);
                } else {
                    recursive_update(node_idx * 2 + 2, mid + 1, end, new_value, tree, idx);
                }
                tree.nodes[node_idx] = tree.nodes[node_idx * 2 + 1] + tree.nodes[node_idx * 2 + 2];
            }
        }

        recursive_update(0, start, end, new_value, self, idx);
    }
    fn sum(&self, start: usize, end: usize) -> T {
        fn recursive_sum<T: Copy + Add<Output = T> + AddAssign + Default>(
            node_idx: usize,
            node_start: usize,
            node_end: usize,
            tree: &SegmentTree<T>,
            start: usize,
            end: usize,
        ) -> T {
            if end < node_start || node_end < start {
                T::default()
            } else if start <= node_start && node_end <= end {
                tree.nodes[node_idx]
            } else {
                let mid = (node_start + node_end) / 2;
                recursive_sum(node_idx * 2 + 1, node_start, mid, tree, start, end)
                    + recursive_sum(node_idx * 2 + 2, mid + 1, node_end, tree, start, end)
            }
        }

        recursive_sum(0, 0, self.n - 1, self, start, end)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i128>);

    let n = input.next().unwrap() as usize;
    let m = input.next().unwrap() as usize;
    let k = input.next().unwrap() as usize;

    let mut nums = Vec::new();
    for _ in 0..n {
        nums.push(input.next().unwrap());
    }
    let mut seg_tree = SegmentTree::from(&nums);
    for _ in 0..(m + k) {
        let command = input.next().unwrap();
        if command == 1 {
            let idx = input.next().unwrap() as usize - 1;
            let val = input.next().unwrap();
            seg_tree.update(idx, val);
        } else {
            let start = input.next().unwrap() as usize - 1;
            let end = input.next().unwrap() as usize - 1;
            writeln!(output, "{}", seg_tree.sum(start, end)).unwrap();
        }
    }
    print!("{}", output);
    Ok(())
}
