// N-Queen
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

struct QueenStatus {
    cols: Vec<bool>,
    minus: Vec<bool>,
    plus: Vec<bool>,
    plus_d: usize,
    n: usize,
}

impl QueenStatus {
    fn locate_queen(&mut self, x: usize, y: usize) {
        self.cols[y] = false;
        self.minus[y + x] = false;
        self.plus[y + self.plus_d - x] = false;
    }
    fn remove_queen(&mut self, x: usize, y: usize) {
        self.cols[y] = true;
        self.minus[y + x] = true;
        self.plus[y + self.plus_d - x] = true;
    }
    fn judge_valid_place(&self, x: usize, y: usize) -> bool {
        if self.plus[y + self.plus_d - x] && self.cols[y] && self.minus[x + y] {
            return true;
        }
        false
    }
}

fn calc_queens(
    left_over: usize,
    last_rows: usize,
    queen_status: &mut QueenStatus,
) -> Result<usize, Box<dyn Error>> {
    let n = queen_status.n;
    if left_over == 0 {
        Ok(1)
    } else {
        let mut sum = 0;
        let xdx = last_rows + 1;
        if xdx < queen_status.n {
            for ydx in 0..n {
                if queen_status.judge_valid_place(xdx, ydx) {
                    queen_status.locate_queen(xdx, ydx);
                    sum += calc_queens(left_over - 1, xdx, queen_status).unwrap();
                    queen_status.remove_queen(xdx, ydx);
                }
            }
        }

        Ok(sum)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let mut queen_status = QueenStatus {
        cols: vec![true; n + 1],
        minus: vec![true; 2 * n - 1],
        plus: vec![true; 2 * n - 1],
        plus_d: n - 1,
        n,
    };
    let mut result = 0;
    for x in 0..n {
        for y in x..n {
            queen_status.locate_queen(x, y);
            result += calc_queens(n - 1, x, &mut queen_status).unwrap();
            queen_status.remove_queen(x, y);
        }
    }

    writeln!(output, "{}", result).unwrap();
    print!("{}", output);
    Ok(())
}
