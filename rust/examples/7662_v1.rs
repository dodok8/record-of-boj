// 이중 우선순위 큐

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    let mut lines = input.lines();

    let t: usize = lines.next().unwrap().parse()?;

    for _ in 0..t {
        let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();
        let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let mut cnt_map: HashMap<i32, i32> = HashMap::new();

        let n: usize = lines.next().unwrap().parse()?;

        for _ in 0..n {
            let line = lines.next().unwrap();
            let mut parts = line.split_whitespace();
            let cmd = parts.next().unwrap();
            let target: i32 = parts.next().unwrap().parse()?;

            if cmd == "I" {
                max_heap.push(target);
                min_heap.push(Reverse(target));
                *cnt_map.entry(target).or_insert(0) += 1;
            } else {
                if target == 1 {
                    if let Some(&max_val) = max_heap.peek() {
                        *cnt_map.entry(max_val).or_insert(0) -= 1;
                        max_heap.pop();
                    }
                } else if let Some(&Reverse(min_val)) = min_heap.peek() {
                    *cnt_map.entry(min_val).or_insert(0) -= 1;
                    min_heap.pop();
                }

                while let Some(&max_val) = max_heap.peek() {
                    if *cnt_map.get(&max_val).unwrap_or(&0) == 0 {
                        max_heap.pop();
                    } else {
                        break;
                    }
                }

                while let Some(&Reverse(min_val)) = min_heap.peek() {
                    if *cnt_map.get(&min_val).unwrap_or(&0) == 0 {
                        min_heap.pop();
                    } else {
                        break;
                    }
                }
            }
        }

        while let Some(&max_val) = max_heap.peek() {
            if *cnt_map.get(&max_val).unwrap_or(&0) == 0 {
                max_heap.pop();
            } else {
                break;
            }
        }

        while let Some(&Reverse(min_val)) = min_heap.peek() {
            if *cnt_map.get(&min_val).unwrap_or(&0) == 0 {
                min_heap.pop();
            } else {
                break;
            }
        }

        if max_heap.is_empty() || min_heap.is_empty() {
            writeln!(output, "EMPTY")?;
        } else {
            let max_val = max_heap.peek().unwrap();
            let min_val = min_heap.peek().unwrap().0;
            writeln!(output, "{} {}", max_val, min_val)?;
        }
    }

    print!("{}", output);
    Ok(())
}
