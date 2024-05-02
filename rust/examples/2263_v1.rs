// 트리의 순회

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn update_preorder(
    preorder: &mut Vec<usize>,
    inorder: &Vec<usize>,
    postorder: &Vec<usize>,
    in_start: usize,
    in_end: usize,
    post_start: usize,
    post_end: usize,
) {
    if in_start == in_end || post_start == post_end {
        preorder.push(postorder[post_end]);
        return;
    } else if in_start > in_end || post_start > post_end {
        return;
    }
    let in_root = inorder[in_start..=in_end]
        .iter()
        .position(|&x| x == postorder[post_end])
        .map(|i| i + in_start)
        .unwrap();
    preorder.push(postorder[post_end]);
    update_preorder(
        preorder,
        inorder,
        postorder,
        in_start,
        in_root - 1,
        post_start,
        post_start + in_root - in_start - 1,
    );
    update_preorder(
        preorder,
        inorder,
        postorder,
        in_root + 1,
        in_end,
        post_start + in_root - in_start,
        post_end - 1,
    );
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let inorder: Vec<usize> = input.clone().take(n).collect();
    let postorder: Vec<usize> = input.skip(n).collect();
    let mut preorder: Vec<usize> = Vec::new();

    update_preorder(&mut preorder, &inorder, &postorder, 0, n - 1, 0, n - 1);

    for node in preorder {
        write!(output, "{} ", node).unwrap();
    }
    println!("{}", output);
    Ok(())
}
