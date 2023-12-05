// 트리의 지름

use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::io::{stdin, Read};

type Weight = usize;
#[derive(Clone)]
struct Edge(Weight, usize);

fn get_longest_path_dfs(edges: &Vec<Vec<Edge>>, start_v: usize, num_v: &usize) -> (Weight, usize) {
    let mut curr_max_v = 0;
    let mut curr_max_w: Weight = 0;
    let mut travel_stack: VecDeque<(usize, Weight)> = VecDeque::new();
    let mut visited = vec![false; num_v + 1];
    travel_stack.push_back((start_v, 0));
    visited[start_v] = true;
    while !travel_stack.is_empty() {
        let (curr_v, curr_w) = travel_stack.pop_back().unwrap();
        for edge in &edges[curr_v] {
            let next_v = edge.1;
            let next_w = edge.0;
            if !visited[next_v] {
                visited[next_v] = true;
                travel_stack.push_back((next_v, curr_w + next_w));
            }
        }
        if curr_max_w < curr_w {
            curr_max_w = curr_w;
            curr_max_v = curr_v;
        }
    }

    (curr_max_w, curr_max_v)
}

/// # 해설
/// DFS 2번으로 풀 수 있는 이유, 증명은 귀류법으로 가능하다.
/// 임의 시작점 x, 여기서 제일 거리먼 지점 y, y에서 제일 거리가 먼 지점을 z라 하자.
/// 그리고 이 둘이 아닌 다른 트리의 지름의 양 끝점 u,v를 잡자
/// y가 두 지점 중 하나가 될 수 밖에 없거나, `d(u,v) == d(y,z)`를 보이면 귀류 증명 완료
/// ## u, v 연결 경로 사이에 x 가 있는 경우
///
/// `d(x,v) <=  d(x,y)`이고, 마찬가지로 `d(x,u) <=  d(x,z)` 이므로 `d(u,v) <= d(y,z)` 이다.
///
/// ## u, v 연결 경로 사이에 x가 없는 경우(일부 경로만 공유하는 경우)
///
/// `d(u,v) = d_u + d_share + d_v`
/// `d(x,y) = d_x + d_share + d_y`
///
/// 이 경우 , `d_v <= d_y` 이어야만 첫번째 DFS에서 y가 선택된다. 즉 이 두 값이 같아야만 `d(u,v)` 가 지름이 될 수 있다.
///
/// ## 아예 경로를 공유 안하는 경우
///
/// 위의 `d_share`가 두 겹치지 않는 경로를 연결하기 위한 무분이 된다. 이를 `d_connect`라 하자.
/// `d_u + d_connect` 와 `d_v + d_connect` 는 `d_y` 보다 작아야 첫번째 DFS에서 y가 선택된다.
/// 즉 이러기 위해서는 값이 같거나, u나 v 중에 y가 있어야 한다.
fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i128>);
    let num_v: usize = input.next().unwrap() as usize;
    let mut edges: Vec<Vec<Edge>> = vec![vec![]; num_v + 1];
    let mut is_leaf = vec![false; num_v + 1];
    let mut leaves = Vec::new();

    for _ in 0..num_v {
        let curr_v = input.next().unwrap() as usize;
        let mut num_adj = 0;

        loop {
            let adj_v = input.next().unwrap();
            if adj_v == -1 {
                if num_adj == 1 {
                    is_leaf[curr_v] = true;
                    leaves.push(curr_v);
                }
                break;
            } else {
                num_adj += 1;
                let weight = input.next().unwrap() as usize;
                edges[curr_v].push(Edge(weight, adj_v as usize));
            }
        }
    }
    let (_first_w, first_v) = get_longest_path_dfs(&edges, 1, &num_v);
    let (max_w, _max_v) = get_longest_path_dfs(&edges, first_v, &num_v);
    writeln!(output, "{}", max_w).unwrap();
    print!("{}", output);
    Ok(())
}
