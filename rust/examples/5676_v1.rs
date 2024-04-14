use std::cmp::Ordering;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

struct SegmentTree {
    nodes: Vec<i32>,
    n: usize,
}
impl SegmentTree {
    fn from(data: &Vec<i32>) -> Self {
        let n = data.len();
        let h = n.next_power_of_two().trailing_zeros() as usize;
        let mut nodes: Vec<i32> = vec![i32::default(); 2_usize.pow(h as u32 + 1)];

        fn fill(
            idx: usize,
            start: usize,
            end: usize,
            nodes: &mut Vec<i32>,
            data: &Vec<i32>,
        ) -> i32 {
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
    fn update(&mut self, idx: usize, new_value: i32) {
        let end = self.n - 1;
        let start = 0_usize;
        fn recursive_update(
            node_idx: usize,
            start: usize,
            end: usize,
            new_value: i32,
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
    fn mul(&self, start: usize, end: usize) -> i32 {
        fn recursive_mul(
            node_idx: usize,
            node_start: usize,
            node_end: usize,
            tree: &SegmentTree,
            start: usize,
            end: usize,
        ) -> i32 {
            if end < node_start || node_end < start {
                1_i32
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
    let mut input = input.split_ascii_whitespace();
    loop {
        let n = match input.next() {
            None => break,
            Some(val) => val.parse::<usize>().unwrap(),
        };
        let m = input.next().unwrap().parse::<usize>().unwrap();
        let mut nums = Vec::new();
        for _ in 0..n {
            let input = input.next().unwrap().parse::<i32>().unwrap();
            nums.push(match input.cmp(&0) {
                Ordering::Greater => 1,
                Ordering::Equal => 0,
                Ordering::Less => -1,
            });
        }

        let mut seg_tree = SegmentTree::from(&nums);
        for _ in 0..m {
            match input.next().unwrap() {
                "C" => {
                    let idx = input.next().unwrap().parse::<usize>().unwrap() - 1;
                    let new_value = match input.next().unwrap().parse::<i32>().unwrap().cmp(&0) {
                        Ordering::Greater => 1,
                        Ordering::Equal => 0,
                        Ordering::Less => -1,
                    };

                    seg_tree.update(idx, new_value);
                }
                "P" => {
                    let start = input.next().unwrap().parse::<usize>().unwrap() - 1;
                    let end = input.next().unwrap().parse::<usize>().unwrap() - 1;
                    let result = seg_tree.mul(start, end);
                    write!(
                        output,
                        "{}",
                        match result.cmp(&0) {
                            Ordering::Greater => "+",
                            Ordering::Equal => "0",
                            Ordering::Less => "-",
                        }
                    )
                    .unwrap();
                }
                _ => unreachable!(),
            }
        }
        writeln!(output).unwrap();
    }
    print!("{}", output);
    Ok(())
}
