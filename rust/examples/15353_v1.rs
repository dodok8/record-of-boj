//큰 수 A+B (2)
//언어 제한으로 인해 Basm 으로 제출함.

use core::fmt;
use std::cmp::max;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};
use std::ops::Add;

struct BigUnsignedInt {
    data: Vec<u8>,
}

impl BigUnsignedInt {
    fn from(s: &str) -> BigUnsignedInt {
        BigUnsignedInt {
            data: s
                .chars()
                .rev()
                .flat_map(|c| c.to_string().parse::<u8>())
                .collect::<Vec<u8>>(),
        }
    }
}

impl Add<BigUnsignedInt> for BigUnsignedInt {
    type Output = BigUnsignedInt;

    fn add(self, rhs: BigUnsignedInt) -> Self::Output {
        let length = max(self.data.len(), rhs.data.len());
        let mut carry = 0;
        let mut result = Vec::new();
        for idx in 0..length {
            let s = *self.data.get(idx).unwrap_or(&0);
            let o = *rhs.data.get(idx).unwrap_or(&0);
            result.push((s + o + carry) % 10);
            carry = (s + o + carry) / 10;
        }
        if carry == 1 {
            result.push(carry);
        }

        BigUnsignedInt { data: result }
    }
}

impl fmt::Display for BigUnsignedInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for digit in self.data.iter().rev() {
            write!(f, "{}", *digit)?;
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let a = BigUnsignedInt::from(input.next().unwrap());
    let b = BigUnsignedInt::from(input.next().unwrap());
    writeln!(output, "{}", a + b).unwrap();
    print!("{}", output);
    Ok(())
}

/* basm
use crate::alloc::string::ToString;
use alloc::vec::Vec;
use basm::platform::io::{Reader, Writer};
use core::{cmp::max, fmt, ops::Add};

struct BigUnsignedInt {
    data: Vec<u8>,
}

impl BigUnsignedInt {
    fn from(s: &str) -> BigUnsignedInt {
        BigUnsignedInt {
            data: s
                .chars()
                .rev()
                .flat_map(|c| c.to_string().parse::<u8>())
                .collect::<Vec<u8>>(),
        }
    }
}

impl Add<BigUnsignedInt> for BigUnsignedInt {
    type Output = BigUnsignedInt;

    fn add(self, rhs: BigUnsignedInt) -> Self::Output {
        let length = max(self.data.len(), rhs.data.len());
        let mut carry = 0;
        let mut result = Vec::new();
        for idx in 0..length {
            let s = *self.data.get(idx).unwrap_or(&0);
            let o = *rhs.data.get(idx).unwrap_or(&0);
            result.push((s + o + carry) % 10);
            carry = (s + o + carry) / 10;
        }
        if carry == 1 {
            result.push(carry);
        }

        BigUnsignedInt { data: result }
    }
}

impl fmt::Display for BigUnsignedInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for digit in self.data.iter().rev() {
            write!(f, "{}", *digit)?;
        }
        Ok(())
    }
}

pub fn main() {
    let mut reader: Reader = Default::default();
    let mut writer: Writer = Default::default();
    let a = BigUnsignedInt::from(&reader.word());
    let b = BigUnsignedInt::from(&reader.word());
    writer.str(&(a + b).to_string());
    writer.byte(b'\n');
}


*/
