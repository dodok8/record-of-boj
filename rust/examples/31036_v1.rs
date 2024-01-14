// 줌
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn operation_1(
    a: &mut Vec<usize>,
    prefix_sum: &mut Vec<usize>,
    l: usize,
    r: usize,
    b: usize,
    n: usize,
    DIV: usize,
) {
    for idx in l..(r + 1) {
        a[idx] = (b + a[idx]) % DIV;
    }
    for idx in l..(n + 1) {
        prefix_sum[idx] = a[idx] + prefix_sum[idx - 1];
    }
}

fn operation_2(a: &mut Vec<usize>, prefix_sum: &mut Vec<usize>, l: usize, n: usize) {
    let mut new_a = vec![0; n + 1];
    for i in 1..(n + 1) {
        let idx = l + ((i - 1) / 2);
        new_a[i] = a[idx];
    }

    *a = new_a;
    for idx in 1..(n + 1) {
        prefix_sum[idx] = a[idx] + prefix_sum[idx - 1];
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    const DIV: usize = 998_244_353;
    let n = input.next().unwrap();
    let num_q = input.next().unwrap();

    let mut prefix_sum = vec![0; n + 1];
    let mut a = vec![0; n + 1];

    let mut queries: Vec<(usize, usize, usize, usize)> = Vec::new();

    for _ in 0..num_q {
        match input.next().unwrap() {
            1 => {
                let l = input.next().unwrap();
                let r = input.next().unwrap();
                let b = input.next().unwrap();
                operation_1(&mut a, &mut prefix_sum, l, r, b, n, DIV);
                queries.push((1, l, r, b));
            }
            2 => {
                let l = input.next().unwrap();
                operation_2(&mut a, &mut prefix_sum, l, n);
                queries.push((2, l, 0, 0));
            }
            3 => {
                let curr_num_q = queries.len();
                for idx in 0..curr_num_q {
                    let (t, l, r, b) = queries[idx];
                    match t {
                        1 => {
                            operation_1(&mut a, &mut prefix_sum, l, r, b, n, DIV);
                        }
                        2 => {
                            operation_2(&mut a, &mut prefix_sum, l, n);
                        }
                        _ => panic!(),
                    }

                    queries.push((t, l, r, b));
                }
            }
            4 => {
                let l = input.next().unwrap();
                let r = input.next().unwrap();
                writeln!(output, "{}", prefix_sum[r] - prefix_sum[l - 1]).unwrap();
            }
            _ => {
                panic!();
            }
        }
    }
    print!("{}", output);
    Ok(())
}
