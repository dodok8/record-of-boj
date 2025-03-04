// 행렬 만들기

use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();

    let source = 1usize; // 1번 노드는 source
    let sink = 2 + 2 * n; // 2+ 2*n 노드는 sink
                          // 0 번 노드는 부모 없음을 표현하기 위해 비워 둔다.

    let mut capacity = vec![vec![0i64; 2 * n + 3]; 2 * n + 3];
    let mut flow = vec![vec![0i64; 2 * n + 3]; 2 * n + 3];
    let mut hangs = Vec::new();
    let mut yuls = Vec::new();
    let mut graph = vec![vec![]; 2 * n + 3];

    for idx in 2..=(n + 1) {
        // 2번 노드 부터 n+1 번 노드는 행에 대응한다.
        let a = input.next().unwrap() as i64;
        graph[source].push(idx);
        graph[idx].push(source);
        capacity[source][idx] = a;
        hangs.push(a);
    }

    for idx in (n + 2)..=(2 * n + 1) {
        // n+2 번 노드 부터 2 * n + 1 노드는 열에 대응한다.
        let a = input.next().unwrap() as i64;
        graph[idx].push(sink);
        graph[sink].push(idx);
        capacity[idx][sink] = a;
        yuls.push(a);
    }

    for idx in 2..=(n + 1) {
        for jdx in (n + 2)..=(2 * n + 1) {
            capacity[idx][jdx] = 1 as i64;
            graph[idx].push(jdx);
            graph[jdx].push(idx);
        }
    }

    loop {
        let mut parents = vec![0; 3 + 2 * n];
        parents[source] = source;

        let mut travel_q = VecDeque::new();
        travel_q.push_back(source);

        while !travel_q.is_empty() && parents[sink] == 0 {
            let curr_v = travel_q.pop_front().unwrap();

            for &next_v in &graph[curr_v] {
                if capacity[curr_v][next_v] - flow[curr_v][next_v] > 0 && parents[next_v] == 0 {
                    travel_q.push_back(next_v);
                    parents[next_v] = curr_v;
                }
            }
        }

        if parents[sink] == 0 {
            break;
        }

        let mut fill = i64::MAX;
        let mut pdx = sink;
        while pdx != source {
            fill = fill.min(capacity[parents[pdx]][pdx] - flow[parents[pdx]][pdx]);
            pdx = parents[pdx];
        }

        let mut pdx = sink;
        while pdx != source {
            flow[parents[pdx]][pdx] += fill; // 증가 경로는 유량 증가!
            flow[pdx][parents[pdx]] -= fill; // 반대는 감소!
            pdx = parents[pdx];
        }
    }

    let mut valid = true;

    for (idx, &hang) in hangs.iter().enumerate() {
        let idx = idx + 2;
        let mut sum = 0;

        for jdx in (n + 2)..=(2 * n + 1) {
            sum += flow[idx][jdx];
        }

        if hang != sum {
            valid = false;
            break;
        }
    }

    for (jdx, &yul) in yuls.iter().enumerate() {
        let jdx = jdx + n + 2;
        let mut sum = 0;

        for idx in 2..=(n + 1) {
            sum += flow[idx][jdx];
        }

        if yul != sum {
            valid = false;
            break;
        }
    }

    if !valid {
        writeln!(output, "-1")?;
    } else {
        writeln!(output, "1")?;
        for idx in 2..=(n + 1) {
            for jdx in (n + 2)..=(2 * n + 1) {
                write!(output, "{}", flow[idx][jdx])?;
            }
            writeln!(output)?;
        }
    }
    print!("{}", output);
    Ok(())
}
