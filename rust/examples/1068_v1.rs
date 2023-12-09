// 트리
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

#[derive(Clone)]
struct Node {
    index: usize,
    children: Vec<usize>,
}

#[derive(Clone)]
struct Tree {
    nodes: Vec<Node>,
    survived: Vec<bool>,
    root_idx: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let num_v: usize = input.next().unwrap() as usize;
    let mut tree: Tree = Tree {
        nodes: vec![
            Node {
                index: 0,
                children: Vec::new(),
            };
            num_v
        ],
        survived: vec![true; num_v],
        root_idx: 0,
    };
    for index in 0..num_v {
        let val = input.next().unwrap();
        match val {
            -1 => {
                tree.nodes[index].index = index;
                tree.root_idx = index;
            }
            _ => {
                let val = val as usize;
                tree.nodes[index].index = index;
                tree.nodes[val].children.push(index);
            }
        }
    }

    let erased_v = input.next().unwrap() as usize;
    let mut delete_stack = VecDeque::new();
    delete_stack.push_back(erased_v);
    while !delete_stack.is_empty() {
        let curr_idx = delete_stack.pop_back().unwrap();
        tree.survived[curr_idx] = false;
        for child in &tree.nodes[curr_idx].children {
            delete_stack.push_back(*child);
        }
    }

    let mut num_leaves = 0;
    if tree.survived[tree.root_idx] {
        let mut travel_stack = VecDeque::new();
        travel_stack.push_back(tree.root_idx);
        while !travel_stack.is_empty() {
            let curr_idx = travel_stack.pop_back().unwrap();
            let mut is_leaf = true;
            if !tree.nodes[curr_idx].children.is_empty() {
                for child in &tree.nodes[curr_idx].children {
                    if tree.survived[*child] {
                        is_leaf = false;
                        travel_stack.push_back(*child);
                    }
                }
            }

            if is_leaf {
                num_leaves += 1;
            }
        }
    }
    writeln!(output, "{}", num_leaves).unwrap();
    print!("{}", output);
    Ok(())
}
