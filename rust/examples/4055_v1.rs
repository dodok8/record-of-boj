// 파티가 좋아 파티가 좋아

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i64>);
    let mut tdx = 1;
    loop {
        let num_p = input.next().unwrap();
        if num_p == 0 {
            break;
        }
        let mut parties = Vec::new();
        for _ in 0..num_p {
            parties.push((input.next().unwrap(), input.next().unwrap()));
        }
        parties.sort_by_key(|p| -p.0);
        parties.sort_by_key(|p| p.1);

        let mut attend = [0; 16];
        let mut cnt = 0;
        for party in parties.iter() {
            for t in party.0..party.1 {
                let t = t as usize;
                if attend[t] < 2 {
                    attend[t] += 1;
                    cnt += 1;
                    break;
                }
            }
        }
        writeln!(
            output,
            "On day {} Emma can attend as many as {} parties.",
            tdx, cnt
        )
        .unwrap();

        tdx += 1;
    }
    print!("{}", output);
    Ok(())
}
