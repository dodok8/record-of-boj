//행렬 곱셈

use std::error::Error;
use std::fmt;
use std::fmt::Write;
use std::io::{stdin, Read};

#[derive(Debug)]
struct UnableToDotProductError;

impl Error for UnableToDotProductError {}

impl fmt::Display for UnableToDotProductError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to dot product")
    }
}

trait Matrix {
    fn dot(&self, other: &Self) -> Result<Self, UnableToDotProductError>
    where
        Self: std::marker::Sized;
}

impl Matrix for Vec<Vec<i32>> {
    fn dot(&self, other: &Self) -> Result<Self, UnableToDotProductError> {
        let (a_n, a_m) = (self.len(), self[0].len());
        let (other_m, other_k) = (other.len(), other[0].len());

        if a_m != other_m {
            return Err(UnableToDotProductError);
        }

        let mut result = vec![vec![0; other_k]; a_n];
        for (idx, row) in result.iter_mut().enumerate() {
            for (jdx, cell) in row.iter_mut().enumerate() {
                *cell = (0..a_m).map(|kdx| self[idx][kdx] * other[kdx][jdx]).sum();
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

    let (m, k) = (
        input.next().unwrap() as usize,
        input.next().unwrap() as usize,
    );
    let mut mat_b = vec![vec![0; k]; m];
    for idx in 0..m {
        for jdx in 0..k {
            mat_b[idx][jdx] = input.next().unwrap();
        }
    }

    for row in mat_a.dot(&mat_b).unwrap() {
        for num in row {
            write!(output, "{} ", num).unwrap();
        }
        writeln!(output).unwrap();
    }
    print!("{}", output);
    Ok(())
}
