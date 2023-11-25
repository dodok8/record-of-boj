// N-Queen
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn locate_queen(board: &mut [Vec<usize>], x: usize, y: usize) -> Result<(), Box<dyn Error>> {
    let n = board.len();
    if board[x][y] > 0 {
        return Err("Queen Exists".into());
    }
    for idx in 0..n {
        board[x][idx] += 1;
        board[idx][y] += 1;
    }
    board[x][y] = 1;
    for (dx, dy) in [(-1_i32, -1_i32), (1, -1), (-1, 1), (1, 1)] {
        let mut curr_x: i32 = x as i32 + dx;
        let mut curr_y: i32 = y as i32 + dy;
        loop {
            if curr_x < 0 || curr_x >= n as i32 || curr_y < 0 || curr_y >= n as i32 {
                break;
            } else {
                board[curr_x as usize][curr_y as usize] += 1;
            }
            curr_x += dx;
            curr_y += dy;
        }
    }
    Ok(())
}

fn remove_queen(board: &mut [Vec<usize>], x: usize, y: usize) -> Result<(), Box<dyn Error>> {
    let n = board.len();
    if board[x][y] == 0 || board[x][y] != 1 {
        return Err("No Queen Exists".into());
    }
    board[x][y] += 1;
    for idx in 0..n {
        board[x][idx] -= 1;
        board[idx][y] -= 1;
    }
    for (dx, dy) in [(-1_i32, -1_i32), (1, -1), (-1, 1), (1, 1)] {
        let mut curr_x: i32 = x as i32 + dx;
        let mut curr_y: i32 = y as i32 + dy;
        loop {
            if curr_x < 0 || curr_x >= n as i32 || curr_y < 0 || curr_y >= n as i32 {
                break;
            } else {
                board[curr_x as usize][curr_y as usize] -= 1;
            }
            curr_x += dx;
            curr_y += dy;
        }
    }
    Ok(())
}

fn calc_queen(
    board: &mut [Vec<usize>],
    last_x: usize,
    last_y: usize,
    num_left_queen: usize,
) -> Result<usize, Box<dyn Error>> {
    let n = board.len();
    if num_left_queen == 0 {
        return Ok(1);
    }
    let mut sum = 0;

    for xdx in last_x..n {
        for ydx in 0..n {
            if !(last_x == 0 && last_y == 0) && xdx == last_x && ydx <= last_y {
                continue;
            }
            if board[xdx][ydx] == 0 {
                locate_queen(board, xdx, ydx).unwrap();
                sum += calc_queen(board, xdx, ydx, num_left_queen - 1).unwrap();
                remove_queen(board, xdx, ydx).unwrap();
            }
        }
    }
    Ok(sum)
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let mut board = vec![vec![0; n]; n];
    writeln!(output, "{}", calc_queen(&mut board, 0, 0, n).unwrap()).unwrap();
    print!("{}", output);
    Ok(())
}
