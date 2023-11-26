use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

type Weight = i32;
type Height = i32;
struct Dungchi(Weight, Height);

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(str::parse::<i32>);
    let num_people = input.next().unwrap() as usize;
    let mut dungchis: Vec<Dungchi> = Vec::new();
    for _ in 0..num_people {
        let weight = input.next().unwrap();
        let height = input.next().unwrap();
        dungchis.push(Dungchi(weight, height));
    }

    for idx in 0..num_people {
        write!(output, "{} ", {
            let mut count = 1;
            for jdx in 0..num_people {
                if dungchis[idx].0 < dungchis[jdx].0 && dungchis[idx].1 < dungchis[jdx].1 {
                    count += 1;
                }
            }
            count
        })
        .unwrap();
    }
    println!("{}", output);
    Ok(())
}
