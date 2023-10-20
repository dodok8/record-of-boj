use std::collections::VecDeque;
use std::error::Error;
use std::fmt::{self, Display, Formatter, Write};
use std::io::{stdin, Read};

struct Node<T: Copy> {
    index: usize,
    data: T,
    left: Option<usize>,
    right: Option<usize>,
    parent: Option<usize>,
}

impl<T> Node<T>
where
    T: Copy,
{
    fn get_inorder(&self, nodes: &Vec<Node<T>>) -> Vec<(usize, T)> {
        let mut travel_queue: VecDeque<usize> = VecDeque::from([self.index]);
        let mut result: Vec<(usize, T)> = Vec::new();
        let mut visited: Vec<bool> = vec![false; nodes.len()];
        while travel_queue.len() != 0 {
            let curr_idx = travel_queue.pop_front().unwrap();
            if visited[curr_idx] {
                break;
            };
            visited[curr_idx] = true;
            let curr_node = &nodes[curr_idx];
            result.push((curr_idx, curr_node.data));
            match curr_node.left {
                Some(value) => travel_queue.push_back(value),
                _ => (),
            }

            match curr_node.right {
                Some(value) => travel_queue.push_back(value),
                _ => (),
            }
        }
        result
    }

    fn get_preorder(&self, nodes: &Vec<Node<T>>) -> Vec<(usize, T)> {}
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .map(str::parse::<i32>)
        .flatten();
    let num_nodes = input.next().unwrap() as usize;
    writeln!(output, "Hello world").unwrap();
    print!("{}", output);
    Ok(())
}
