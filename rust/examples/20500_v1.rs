// Ezreal 여눈부터 가네 ㅈㅈ

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn get_cache(
    end: usize,
    rest: usize,
    length: usize,
    cache: &mut Vec<Vec<Vec<Option<usize>>>>,
) -> usize {
    let end = end % 5;
    if let Some(val) = cache[end][rest][length] {
        val
    } else {
        cache[end][rest][length] = Some(
            (get_cache(
                1,
                (rest + 6 - if end == 0 { 5 } else { 1 }) % 3,
                length - 1,
                cache,
            ) + get_cache(
                5,
                (rest + 6 - if end == 0 { 5 } else { 1 }) % 3,
                length - 1,
                cache,
            )) % 1_000_000_007,
        );

        cache[end][rest][length].unwrap()
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = input.next().unwrap();
    let mut cache: Vec<Vec<Vec<Option<usize>>>> = vec![vec![vec![None; n + 1]; 3]; 2];
    // 1로 끝나는 길이 1인 애들 초기화
    cache[1][0][1] = Some(0);
    cache[1][1][1] = Some(1);
    cache[1][2][1] = Some(0);
    // 5로 끝나는 길이 1인 애들 초기화
    cache[0][0][1] = Some(0);
    cache[0][1][1] = Some(0);
    cache[0][2][1] = Some(1);

    let result = get_cache(5, 0, n, &mut cache);
    writeln!(output, "{}", result).unwrap();

    print!("{}", output);
    Ok(())
}
