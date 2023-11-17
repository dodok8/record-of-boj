// 별 찍기 - 11
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn fill_star(x: usize, y: usize, size: usize, board: &mut Vec<Vec<char>>) {
    if size == 3 {
        board[x][y] = '*';
        board[x + 1][y - 1] = '*';
        board[x + 1][y + 1] = '*';
        for del in 0..3 {
            board[x + 2][y + del as usize] = '*';
            board[x + 2][y - del as usize] = '*';
        }
    } else {
        fill_star(x, y, size / 2, board);
        fill_star(x + size / 2, y - size / 2, size / 2, board);
        fill_star(x + size / 2, y + size / 2, size / 2, board);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let d = 2 * n - 1;
    let mut board = vec![vec![' '; d]; n];
    fill_star(0, n - 1, n, &mut board);
    for row in board.iter() {
        for cell in row.iter() {
            write!(output, "{}", cell).unwrap();
        }
        writeln!(output).unwrap();
    }
    println!("{}", output);
    Ok(())
}
