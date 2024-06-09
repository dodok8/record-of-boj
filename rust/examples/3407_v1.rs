// 맹세

use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let c1 = [
        "h", "b", "c", "n", "o", "f", "p", "s", "k", "v", "y", "i", "w", "u",
    ];

    let c2 = [
        "ba", "ca", "ga", "la", "na", "pa", "ra", "ta", "db", "nb", "pb", "rb", "sb", "tb", "yb",
        "ac", "sc", "tc", "cd", "gd", "md", "nd", "pd", "be", "ce", "fe", "ge", "he", "ne", "re",
        "se", "te", "xe", "cf", "hf", "rf", "ag", "hg", "mg", "rg", "sg", "bh", "rh", "th", "bi",
        "li", "ni", "si", "ti", "bk", "al", "cl", "fl", "tl", "am", "cm", "fm", "pm", "sm", "tm",
        "cn", "in", "mn", "rn", "sn", "zn", "co", "ho", "mo", "no", "po", "np", "ar", "br", "cr",
        "er", "fr", "ir", "kr", "lr", "pr", "sr", "zr", "as", "cs", "ds", "es", "hs", "os", "at",
        "mt", "pt", "au", "cu", "eu", "lu", "pu", "ru", "lv", "dy",
    ];

    let num_test = input.next().unwrap().parse::<usize>()?;

    for _ in 0..num_test {
        let words = input.next().unwrap();
        let mut visited = vec![false; words.len() + 1];
        let mut travel_q = VecDeque::new();
        travel_q.push_back(0);
        visited[0] = true;

        while !travel_q.is_empty() {
            let current = travel_q.pop_front().unwrap();
            if current == words.len() {
                break;
            }

            for idx in 0..c1.len() {
                if words[current..].starts_with(c1[idx]) {
                    let next = current + c1[idx].len();
                    if !visited[next] {
                        travel_q.push_back(next);
                        visited[next] = true;
                    }
                }
            }

            for idx in 0..c2.len() {
                if words[current..].starts_with(c2[idx]) {
                    let next = current + c2[idx].len();
                    if !visited[next] {
                        travel_q.push_back(next);
                        visited[next] = true;
                    }
                }
            }
        }

        if travel_q.is_empty() && !visited[words.len()] {
            writeln!(output, "NO").unwrap();
        } else {
            writeln!(output, "YES").unwrap();
        }
    }
    print!("{}", output);
    Ok(())
}
