// 수열과 쿼리 16

use std::cmp::min;
use std::cmp::Ord;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

struct SegmentTree<T: Copy> {
    nodes: Vec<T>,
    n: usize,
}

impl<T> SegmentTree<T>
where
    T: Copy + Default + Ord,
{
    fn from(data: &Vec<T>) -> Self {
        let n = data.len();
        let h = n.next_power_of_two().trailing_zeros() as usize;
        let mut nodes: Vec<T> = vec![T::default(); 2_usize.pow(h as u32 + 1)];

        fn fill<T: Copy + Ord>(
            idx: usize,
            start: usize,
            end: usize,
            nodes: &mut Vec<T>,
            data: &Vec<T>,
        ) -> T {
            if start == end {
                nodes[idx] = data[start];
                return nodes[idx];
            }

            let mid = (start + end) / 2;
            nodes[idx] = min(
                fill(idx * 2 + 1, start, mid, nodes, data),
                fill(idx * 2 + 2, mid + 1, end, nodes, data),
            );
            nodes[idx]
        }

        fill(0, 0, data.len() - 1, &mut nodes, data);

        SegmentTree {
            nodes,
            n: data.len(),
        }
    }
    fn update(&mut self, idx: usize, new_value: T) {
        let end = self.n - 1;
        let start = 0_usize;
        fn recursive_update<T: Copy + Ord>(
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
                tree.nodes[node_idx] =
                    min(tree.nodes[node_idx * 2 + 1], tree.nodes[node_idx * 2 + 2]);
            }
        }

        recursive_update(0, start, end, new_value, self, idx);
    }
    fn min(&self, start: usize, end: usize, neutral: T) -> T {
        fn recursive_min<T: Copy + Default + Ord>(
            node_idx: usize,
            node_start: usize,
            node_end: usize,
            tree: &SegmentTree<T>,
            start: usize,
            end: usize,
            neutral: T,
        ) -> T {
            if end < node_start || node_end < start {
                neutral
            } else if start <= node_start && node_end <= end {
                tree.nodes[node_idx]
            } else {
                let mid = (node_start + node_end) / 2;
                min(
                    recursive_min(node_idx * 2 + 1, node_start, mid, tree, start, end, neutral),
                    recursive_min(
                        node_idx * 2 + 2,
                        mid + 1,
                        node_end,
                        tree,
                        start,
                        end,
                        neutral,
                    ),
                )
            }
        }

        recursive_min(0, 0, self.n - 1, self, start, end, neutral)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();

    let mut nums = Vec::new();
    for _ in 0..n {
        nums.push(input.next().unwrap());
    }

    let mut seg_tree = SegmentTree::from(&nums);
    let m = input.next().unwrap();
    for _ in 0..m {
        let command = input.next().unwrap();
        if command == 1 {
            let idx = input.next().unwrap() - 1;
            let val = input.next().unwrap();
            seg_tree.update(idx, val);
            nums[idx] = val;
        } else {
            let start = input.next().unwrap() - 1;
            let end = input.next().unwrap() - 1;
            let min_val = seg_tree.min(start, end, usize::MAX);

            writeln!(
                output,
                "{}",
                start + nums.iter().skip(start).position(|&r| r == min_val).unwrap() + 1
            )
            .unwrap();
        }
    }
    print!("{}", output);
    Ok(())
}
