use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    x: usize,
    y: usize,
    cost: u64,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // reverse ordering
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);

    let v: Vec<Vec<u8>> = reader
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| l.bytes().map(|b| b - b'0').collect::<Vec<u8>>())
        .collect();

    let mut vv: Vec<Vec<u8>> = vec![Vec::with_capacity(v[0].len() * 5); v.len() * 5];
    for i in 0..5 {
        for j in 0..5 {
            for (y, row) in v.iter().enumerate() {
                let mut nrow: Vec<u8> = row
                    .iter()
                    .map(|c| (*c + i as u8 + j as u8 - 1) % 9 + 1)
                    .collect();
                vv[v.len() * i + y].append(&mut nrow);
            }
        }
    }
    let v = vv;

    let mut queue = BinaryHeap::new();
    let mut visited: Vec<Vec<Option<u64>>> = v.iter().map(|v| vec![None; v.len()]).collect();

    let initial = State {
        x: 0,
        y: 0,
        cost: 0,
    };
    visited[0][0] = Some(0);
    queue.push(initial);

    let cost = loop {
        let s = queue.pop().unwrap();
        if let Some(cost) = visited[s.y][s.x] {
            if s.cost > cost {
                continue;
            }
            if s.cost < cost {
                panic!("shouldn't happen");
            }
        }
        if s.y == v.len() - 1 && s.x == v[s.y].len() - 1 {
            break s.cost;
        }
        for (dx, dy) in vec![(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let y = s.y as i32 + dy;
            if y < 0 || y as usize >= v.len() {
                continue;
            }
            let y = y as usize;
            let x = s.x as i32 + dx;
            if x < 0 || x as usize >= v[y].len() {
                continue;
            }
            let x = x as usize;
            let cost = s.cost + v[y][x] as u64;
            if let Some(c) = visited[y][x] {
                if c <= cost {
                    continue;
                }
            }
            visited[y][x] = Some(cost);
            queue.push(State { x, y, cost });
        }
    };
    println!("{}", cost);
}
