// 초정 중성 종성

use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let letter = input.next().unwrap().chars().next().unwrap() as u32 - '가' as u32;
    let cho = [
        'ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ',
        'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ',
    ][(letter / (21 * 28)) as usize];
    let jung = char::from_u32(letter % (21 * 28) / 28 + 0x0000314F).unwrap();

    writeln!(output, "{}", cho).unwrap();
    writeln!(output, "{}", jung).unwrap();
    if letter % 28 != 0 {
        let jong = [
            'ㄱ', 'ㄲ', 'ㄳ', 'ㄴ', 'ㄵ', 'ㄶ', 'ㄷ', 'ㄹ', 'ㄺ', 'ㄻ', 'ㄼ', 'ㄽ', 'ㄾ', 'ㄿ',
            'ㅀ', 'ㅁ', 'ㅂ', 'ㅄ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ',
        ][(letter % 28 - 1) as usize];

        writeln!(output, "{}", jong).unwrap();
    } else {
        writeln!(output).unwrap();
    }

    print!("{}", output);
    Ok(())
}
