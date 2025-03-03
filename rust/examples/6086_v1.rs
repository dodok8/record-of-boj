// 최대 유량

use std::cmp::min;
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<usize>()?;

    let mut capacity = vec![vec![0; 27]; 27];
    let mut flow = vec![vec![0; 27]; 27];

    let convert_letter = |x: &str| (x.chars().take(1).next().unwrap() as u8 - b'A' + 1) as usize;

    for _ in 0..n {
        let start = convert_letter(input.next().unwrap());
        let end = convert_letter(input.next().unwrap());
        let c = input.next().unwrap().parse::<i64>()?;

        capacity[start][end] += c;
        capacity[end][start] += c;
        // 단 방향이랑 다르게 양 방향 다 넣어야 한다.
    }

    let source = convert_letter("A");
    let sink = convert_letter("Z");
    let mut tot_flow = 0;

    loop {
        let mut parents = vec![0; 27];
        let mut travel_q = VecDeque::new();

        parents[source] = source;
        travel_q.push_back(source);

        while !travel_q.is_empty() && parents[sink] == 0 {
            let curr_v = travel_q.pop_front().unwrap();

            for next_v in 1..27 {
                if capacity[curr_v][next_v] - flow[curr_v][next_v] > 0 && parents[next_v] == 0 {
                    travel_q.push_back(next_v);
                    parents[next_v] = curr_v;
                }
            }
        }

        if parents[sink] == 0 {
            // 증가 경로가 사라지는 시점.
            break;
        }

        // 증가 경로에 따라 flow 값을 업데이트 하는 시점 => 경로 중 최소 잔여 용량을 넣어야 하므로 경로를 다 구한 다음에 넣어야 함.

        let mut fill = i64::MAX;

        let mut pdx = sink;
        while pdx != source {
            fill = min(capacity[parents[pdx]][pdx] - flow[parents[pdx]][pdx], fill);
            pdx = parents[pdx];
        }

        let mut pdx = sink;
        while pdx != source {
            flow[parents[pdx]][pdx] += fill; // 증가 경로는 유량 증가!
            flow[pdx][parents[pdx]] -= fill; // 반대는 감소!
            pdx = parents[pdx];
        }

        tot_flow += fill;
    }

    writeln!(output, "{}", tot_flow)?;
    print!("{}", output);
    Ok(())
}
