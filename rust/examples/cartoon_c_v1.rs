// 생일 축하합니다~

use std::error::Error;
use std::io::stdin;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>()?;
    let mut people = Vec::new();
    for _ in 0..n {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        people.push(input);
    }
    for (idx, person) in people.iter().enumerate() {
        println!("? {}", person);
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let first = input.trim().parse::<usize>()?;
        println!("? {}", person);
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let second = input.trim().parse::<usize>()?;
        if first == second && first == 1 {
            println!("! {}", person);
            break;
        }
        if first != second {
            if idx == n - 1 {
                println!("! {}", person);
                break;
            }
            println!("? {}", person);
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            let third = input.trim().parse::<usize>()?;
            if third == 1 {
                println!("! {}", person);
                break;
            }
        }
    }
    Ok(())
}
