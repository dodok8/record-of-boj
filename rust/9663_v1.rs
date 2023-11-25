// N-Queen
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn locate_queen(queens: &mut VecDeque<(usize, usize)>, coord: (usize, usize)) {
    queens.push_back(coord);
}

fn remove_queen(queens: &mut VecDeque<(usize, usize)>) {
    queens.pop_back();
}

fn judge_valid_place(queens: &mut VecDeque<(usize, usize)>, coord: (usize, usize)) -> bool {
    if queens.is_empty() {
        return true;
    }
    queens
        .iter()
        .map(|queen| {
            if coord.0 == queen.0 || coord.1 == queen.1 {
                return false;
            }
            if coord.0.abs_diff(queen.0) == coord.1.abs_diff(queen.1) {
                return false;
            }
            true
        })
        .reduce(|acc, curr| acc && curr)
        .unwrap()
}

fn calc_queen(
    queens: &mut VecDeque<(usize, usize)>,
    n: usize,
    last_x: usize,
    num_left_queen: usize,
    result: &mut usize,
) {
    if num_left_queen == 0 {
        *result += 1;
    } else {
        for xdx in last_x..n {
            for ydx in 0..n {
                if xdx == last_x && (xdx != 0) {
                    continue;
                }
                if judge_valid_place(queens, (xdx, ydx)) {
                    locate_queen(queens, (xdx, ydx));
                    calc_queen(queens, n, xdx, num_left_queen - 1, result);
                    remove_queen(queens);
                }
            }
        }
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let mut queens = VecDeque::new();
    let mut result = 0;
    calc_queen(&mut queens, n, 0, n, &mut result);
    writeln!(output, "{}", result).unwrap();
    print!("{}", output);
    Ok(())
}
