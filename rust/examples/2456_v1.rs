// 나는 학급회장이다.

use std::cmp::Ordering;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

#[derive(Debug, Eq, Clone)]
struct Score(usize, usize, usize, usize);

impl PartialEq for Score {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.0.cmp(&other.0) {
            Ordering::Equal => match self.1.cmp(&other.1) {
                Ordering::Equal => self.2.cmp(&other.2),
                ord => ord,
            },
            ord => ord,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let mut scores = vec![Score(0, 0, 0, 0); 3];

    for _ in 0..n {
        for idx in 0..3 {
            let score = input.next().unwrap();
            scores[idx].0 += score;
            match score {
                3 => scores[idx].1 += 1,
                2 => scores[idx].2 += 1,
                1 => scores[idx].3 += 1,
                _ => {}
            }
        }
    }

    let max_score = scores.iter().max().unwrap();
    let winners: Vec<_> = scores
        .iter()
        .enumerate()
        .filter(|(_, score)| score == &max_score)
        .collect();

    if winners.len() == 1 {
        writeln!(output, "{} {}", winners[0].0 + 1, winners[0].1 .0)?;
    } else {
        writeln!(output, "0 {}", max_score.0)?;
    }

    print!("{}", output);
    Ok(())
}
