// TODO: 이진 검색 트리
use std::collections::VecDeque;
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
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut tree: BinaryTree<i32> = BinaryTree {
        nodes: Vec::new(),
        root_idx: 0,
    };
    let mut check_stack = VecDeque::new();
    {
        let given_num = input.next().unwrap();
        tree.nodes.push(Node {
            index: 0,
            data: given_num,
            left: None,
            right: None,
            parent: None,
        });
        check_stack.push_back((given_num, 0_usize));
    }
    for (idx, given_num) in (1..).zip(input) {
        if check_stack.back().unwrap().0 > given_num {
            tree.nodes.push(Node {
                index: idx,
                data: given_num,
                left: None,
                right: None,
                parent: Some(check_stack.back().unwrap().1),
            });
            tree.nodes[check_stack.back().unwrap().1].left = Some(idx);
            check_stack.push_back((given_num, idx));
        } else {
            let curr_back = loop {
                if check_stack.back().unwrap().0 > given_num {
                    break check_stack.pop_back().unwrap();
                }
            };
            tree.nodes.push(Node {
                index: idx,
                data: given_num,
                left: None,
                right: None,
                parent: Some(curr_back.1),
            });
            tree.nodes[curr_back.1].right = Some(idx);
            check_stack.push_back((given_num, idx));
        }
    }
    for data_idx in tree.get_postorder() {
        writeln!(output, "{}", tree.nodes[data_idx].data).unwrap();
    }

    print!("{}", output);
    Ok(())
}
