//행렬 덧셈

use std::error::Error;
use std::fmt;
use std::fmt::Write;
use std::io::{stdin, Read};

#[derive(Debug)]
struct UnableToPlusError;

impl Error for UnableToPlusError {}

impl fmt::Display for UnableToPlusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to dot product")
    }
}

trait Matrix {
    fn plus(&self, other: &Self) -> Result<Self, UnableToPlusError>
    where
        Self: std::marker::Sized;
}

impl Matrix for Vec<Vec<i32>> {
    fn plus(&self, other: &Self) -> Result<Self, UnableToPlusError> {
        let (a_n, a_m) = (self.len(), self[0].len());
        let (b_n, b_m) = (other.len(), other[0].len());

        if a_n != b_n || a_m != b_m {
            return Err(UnableToPlusError);
        }

        let mut result = vec![vec![0; a_m]; a_n];
        for (idx, row) in result.iter_mut().enumerate() {
            for (jdx, cell) in row.iter_mut().enumerate() {
                *cell = self[idx][jdx] + other[idx][jdx];
            }
        }
        Ok(result)
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let (n, m) = (
        input.next().unwrap() as usize,
        input.next().unwrap() as usize,
    );
    let mut mat_a = vec![vec![0; m]; n];
    for idx in 0..n {
        for jdx in 0..m {
            mat_a[idx][jdx] = input.next().unwrap();
        }
    }
    let mut mat_b = vec![vec![0; m]; n];
    for idx in 0..n {
        for jdx in 0..m {
            mat_b[idx][jdx] = input.next().unwrap();
        }
    }

    for row in mat_a.plus(&mat_b).unwrap() {
        for num in row {
            write!(output, "{} ", num).unwrap();
        }
        writeln!(output).unwrap();
    }
    print!("{}", output);
    Ok(())
}
