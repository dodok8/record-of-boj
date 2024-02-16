// 과제 안 내신 분..?
use std::collections::HashSet;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};
use std::iter::FromIterator;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut set: HashSet<usize> = HashSet::from_iter(1..31);
    for _ in 1..29 {
        let student = input.next().unwrap();
        set.remove(&student);
    }

    let mut vec: Vec<_> = set.into_iter().collect();
    vec.sort_unstable();

    write!(output, "{}\n{}\n", vec[0], vec[1]).unwrap();
    print!("{}", output);
    Ok(())
}
