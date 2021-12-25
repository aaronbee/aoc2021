use std::collections::BinaryHeap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn explore_basin(b: &Vec<Vec<u8>>, e: &mut Vec<Vec<bool>>, x: usize, y: usize) -> u32 {
    if e[y][x] {
        return 0;
    }
    e[y][x] = true;
    let mut ret = 1;
    if x > 0 {
        ret += explore_basin(b, e, x - 1, y);
    }
    if x + 1 < b[y].len() {
        ret += explore_basin(b, e, x + 1, y);
    }
    if y > 0 {
        ret += explore_basin(b, e, x, y - 1);
    }
    if y + 1 < b.len() {
        ret += explore_basin(b, e, x, y + 1);
    }
    return ret;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);

    let v: Vec<Vec<u8>> = reader
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| l.bytes().map(|c| c - '0' as u8).collect::<Vec<u8>>())
        .collect();

    let mut explored: Vec<Vec<bool>> = v
        .iter()
        .map(|v| v.iter().map(|c| *c == 9).collect())
        .collect();

    let mut heap = BinaryHeap::new();

    for y in 0..v.len() {
        for x in 0..v[y].len() {
            if explored[y][x] {
                continue;
            }
            heap.push(explore_basin(&v, &mut explored, x, y));
        }
    }

    println!(
        "{}",
        heap.pop().unwrap() * heap.pop().unwrap() * heap.pop().unwrap()
    );
}
