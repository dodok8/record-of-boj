// 구간 곱 구하기

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

struct SegmentTree {
    nodes: Vec<usize>,
    n: usize,
}

impl SegmentTree {
    fn from(data: &Vec<usize>) -> Self {
        let n = data.len();
        let h = n.next_power_of_two().trailing_zeros() as usize;
        let mut nodes: Vec<usize> = vec![usize::default(); 2_usize.pow(h as u32 + 1)];

        fn fill(
            idx: usize,
            start: usize,
            end: usize,
            nodes: &mut Vec<usize>,
            data: &Vec<usize>,
        ) -> usize {
            if start == end {
                nodes[idx] = data[start];
                return nodes[idx] % 1_000_000_007;
            }

            let mid = (start + end) / 2;
            nodes[idx] = fill(idx * 2 + 1, start, mid, nodes, data)
                * fill(idx * 2 + 2, mid + 1, end, nodes, data)
                % 1_000_000_007;
            nodes[idx]
        }

        fill(0, 0, data.len() - 1, &mut nodes, data);

        SegmentTree {
            nodes,
            n: data.len(),
        }
    }
    fn update(&mut self, idx: usize, new_value: usize) {
        let end = self.n - 1;
        let start = 0_usize;
        fn recursive_update(
            node_idx: usize,
            start: usize,
            end: usize,
            new_value: usize,
            tree: &mut SegmentTree,
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
                tree.nodes[node_idx] = tree.nodes[node_idx * 2 + 1] % 1_000_000_007
                    * tree.nodes[node_idx * 2 + 2]
                    % 1_000_000_007;
            }
        }

        recursive_update(0, start, end, new_value, self, idx);
    }
    fn mul(&self, start: usize, end: usize) -> usize {
        fn recursive_mul(
            node_idx: usize,
            node_start: usize,
            node_end: usize,
            tree: &SegmentTree,
            start: usize,
            end: usize,
        ) -> usize {
            if end < node_start || node_end < start {
                1_usize
            } else if start <= node_start && node_end <= end {
                tree.nodes[node_idx] % 1_000_000_007
            } else {
                let mid = (node_start + node_end) / 2;
                recursive_mul(node_idx * 2 + 1, node_start, mid, tree, start, end) % 1_000_000_007
                    * recursive_mul(node_idx * 2 + 2, mid + 1, node_end, tree, start, end)
                    % 1_000_000_007
            }
        }

        recursive_mul(0, 0, self.n - 1, self, start, end) % 1_000_000_007
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let m = input.next().unwrap();
    let k = input.next().unwrap();

    let mut nums = Vec::new();
    for _ in 0..n {
        nums.push(input.next().unwrap());
    }
    let mut seg_tree = SegmentTree::from(&nums);
    for _ in 0..(m + k) {
        let command = input.next().unwrap();
        if command == 1 {
            let idx = input.next().unwrap() - 1;
            let val = input.next().unwrap();
            seg_tree.update(idx, val);
        } else {
            let start = input.next().unwrap() - 1;
            let end = input.next().unwrap() - 1;
            writeln!(output, "{}", seg_tree.mul(start, end)).unwrap();
        }
    }
    print!("{}", output);
    Ok(())
}
