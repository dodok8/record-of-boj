// 후위 표기식
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

#[derive(Clone)]
struct Node<T: Copy> {
    index: usize,
    data: T,
    left: Option<usize>,
    right: Option<usize>,
    parent: Option<usize>,
}

#[derive(Clone)]
struct BinaryTree<T: Copy> {
    nodes: Vec<Node<T>>,
    root_idx: usize,
}

impl<T> BinaryTree<T>
where
    T: Copy,
{
    fn get_preorder(&self) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();

        fn _preorder<T: Copy>(curr_idx: usize, nodes: &Vec<Node<T>>, result: &mut Vec<usize>) {
            result.push(curr_idx);
            if let Some(value) = nodes[curr_idx].left {
                _preorder(value, nodes, result)
            }
            if let Some(value) = nodes[curr_idx].right {
                _preorder(value, nodes, result)
            }
        }

        _preorder(self.root_idx, &self.nodes, &mut result);
        result
    }

    fn get_inorder(&self) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();

        fn _inorder<T: Copy>(curr_idx: usize, nodes: &Vec<Node<T>>, result: &mut Vec<usize>) {
            if let Some(value) = nodes[curr_idx].left {
                _inorder(value, nodes, result)
            }
            result.push(curr_idx);
            if let Some(value) = nodes[curr_idx].right {
                _inorder(value, nodes, result)
            }
        }

        _inorder(self.root_idx, &self.nodes, &mut result);
        result
    }
    fn get_postorder(&self) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();

        fn _postorder<T: Copy>(curr_idx: usize, nodes: &Vec<Node<T>>, result: &mut Vec<usize>) {
            if let Some(value) = nodes[curr_idx].left {
                _postorder(value, nodes, result)
            }
            if let Some(value) = nodes[curr_idx].right {
                _postorder(value, nodes, result)
            }
            result.push(curr_idx);
        }

        _postorder(self.root_idx, &self.nodes, &mut result);
        result
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let num_nodes: usize = tokens.next().unwrap().parse().unwrap();
    let mut tree: BinaryTree<usize> = BinaryTree {
        nodes: Vec::new(),
        root_idx: 0,
    };
    for idx in 0..26 {
        tree.nodes.push(Node {
            index: idx,
            data: idx,
            left: None,
            right: None,
            parent: None,
        })
    }
    for _idx in 0..num_nodes {
        let node_idx = get_alphabet_order(tokens.next().unwrap().chars().next().unwrap()).unwrap();
        if let Some(value) = get_alphabet_order(tokens.next().unwrap().chars().next().unwrap()) {
            tree.nodes[node_idx].left = Some(value);
            tree.nodes[value].parent = Some(node_idx);
        };
        if let Some(value) = get_alphabet_order(tokens.next().unwrap().chars().next().unwrap()) {
            tree.nodes[node_idx].right = Some(value);
            tree.nodes[value].parent = Some(node_idx);
        };
    }
    for num in tree.get_preorder() {
        write!(output, "{}", get_alphabet_from_num(num)).unwrap();
    }
    writeln!(output).unwrap();
    for num in tree.get_inorder() {
        write!(output, "{}", get_alphabet_from_num(num)).unwrap();
    }
    writeln!(output).unwrap();
    for num in tree.get_postorder() {
        write!(output, "{}", get_alphabet_from_num(num)).unwrap();
    }
    writeln!(output).unwrap();

    println!("{}", output);
    Ok(())
}

fn get_alphabet_order(alphabet: char) -> Option<usize> {
    if alphabet == '.' {
        None
    } else {
        Some(alphabet as u32 as usize - 65)
    }
}

fn get_alphabet_from_num(num: usize) -> char {
    (num + 65) as u8 as char
}
