// 스도쿠

use std::error::Error;
use std::io::{stdin, Read};

fn print_sudoku(sudoku: &Vec<Vec<usize>>) {
    for row in sudoku {
        for num in row {
            print!("{}", num);
        }
        println!();
    }
}
fn get_possible(x: usize, y: usize, sudoku: &Vec<Vec<usize>>) -> Vec<usize> {
    if sudoku[x][y] != 0 {
        return vec![sudoku[x][y]];
    }
    let mut possible = [true; 9];

    for idx in 0..9 {
        if sudoku[x][idx] != 0 {
            possible[sudoku[x][idx] - 1] = false;
        }
        if sudoku[idx][y] != 0 {
            possible[sudoku[idx][y] - 1] = false;
        }

        let nx = x / 3 * 3 + idx / 3;
        let ny = y / 3 * 3 + idx % 3;
        if sudoku[nx][ny] != 0 {
            possible[sudoku[nx][ny] - 1] = false;
        }
    }

    possible
        .iter()
        .enumerate()
        .flat_map(|(idx, &b)| if b { Some(idx + 1) } else { None })
        .collect::<Vec<usize>>()
}

fn fill_sudoku(idx: usize, sudoku: &mut Vec<Vec<usize>>) -> bool {
    if idx < 81 {
        let x = idx / 9;
        let y = idx % 9;

        let is_printed = sudoku[x][y] != 0;
        for num in get_possible(x, y, sudoku) {
            sudoku[x][y] = num;

            if fill_sudoku(idx + 1, sudoku) {
                return true;
            } else {
                if !is_printed {
                    sudoku[x][y] = 0;
                }
                continue;
            };
        }
        false
    } else {
        true
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let mut sudoku = Vec::new();
    for _ in 0..9 {
        sudoku.push(
            input
                .next()
                .unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>(),
        );
    }
    fill_sudoku(0, &mut sudoku);
    print_sudoku(&sudoku);
    Ok(())
}
