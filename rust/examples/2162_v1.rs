// 선분 그룹

use std::cmp;
use std::error::Error;
use std::io::{stdin, Read};

type Point = (i128, i128);

fn get_ccw(p1: &Point, p2: &Point, p3: &Point) -> i128 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let (x3, y3) = p3;

    match (x1 * y2 + x2 * y3 + x3 * y1 - (x2 * y1 + x3 * y2 + x1 * y3)).cmp(&0) {
        cmp::Ordering::Less => -1,
        cmp::Ordering::Equal => 0,
        cmp::Ordering::Greater => 1,
    }
}

fn judge_crossed(first: &(Point, Point), second: &(Point, Point)) -> bool {
    let &(p1, p2) = first;
    let &(p3, p4) = second;

    let mut crossed = false;

    if p1 == p3 || p1 == p4 || p2 == p3 || p2 == p4 {
        crossed = true;
    } else {
        let ccw_12 = get_ccw(&p1, &p2, &p3) * get_ccw(&p1, &p2, &p4);
        let ccw_34 = get_ccw(&p3, &p4, &p1) * get_ccw(&p3, &p4, &p2);

        if ccw_12 == 0 && ccw_34 == 0 {
            if cmp::min(p1.0, p2.0) <= cmp::max(p3.0, p4.0)
                && cmp::min(p3.0, p4.0) <= cmp::max(p1.0, p2.0)
                && cmp::min(p1.1, p2.1) <= cmp::max(p3.1, p4.1)
                && cmp::min(p3.1, p4.1) <= cmp::max(p1.1, p2.1)
            {
                crossed = true;
            }
        } else {
            crossed = ccw_12 <= 0 && ccw_34 <= 0;
        }
    }

    crossed
}

fn find_parent(parents: &mut Vec<usize>, idx: usize) -> usize {
    if idx != parents[idx] {
        parents[idx] = find_parent(parents, parents[idx]);
    }
    parents[idx]
}

fn union(x: usize, y: usize, parents: &mut Vec<usize>) {
    let x = find_parent(parents, x);
    let y = find_parent(parents, y);
    if x < y {
        parents[y] = x;
    } else {
        parents[x] = y;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i128>);

    let n = input.next().unwrap() as usize;
    let mut parents = (0..n).collect::<Vec<usize>>();
    let mut lines: Vec<(Point, Point)> = Vec::new();
    for _ in 0..n {
        lines.push((
            (input.next().unwrap(), input.next().unwrap()),
            (input.next().unwrap(), input.next().unwrap()),
        ));
    }

    for idx in 0..n {
        for jdx in 0..idx {
            if judge_crossed(&lines[jdx], &lines[idx]) {
                union(idx, jdx, &mut parents);
            }
        }
    }

    // Parent가 가장 작은 값이 되도록 전체 Parent를 한번 다시 찾아준다.
    for idx in 0..n {
        find_parent(&mut parents, idx);
    }

    let mut counts = vec![0; n];

    for parent in parents {
        counts[parent] += 1;
    }

    println!("{}", counts.iter().filter(|&&x| x != 0).count());
    println!("{}", counts.iter().max().unwrap());

    Ok(())
}
