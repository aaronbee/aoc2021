use std::cmp;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);

    let mut data: Vec<Vec<u32>> = Vec::new();

    for l in reader.lines().filter_map(|l| l.ok()) {
        let (left, right) = match l.split_once(" -> ") {
            Some(t) => t,
            None => continue,
        };
        let (x, y) = left.split_once(',').unwrap();
        let (x1, y1) = (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());
        let (x, y) = right.split_once(',').unwrap();
        let (x2, y2) = (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());

        if x1 == x2 {
            let (y1, y2) = (cmp::min(y1, y2), cmp::max(y1, y2) + 1);
            if data.len() < y2 {
                data.resize(y2, Vec::new());
            }
            for y in y1..y2 {
                if data[y].len() < (x1 + 1) {
                    data[y].resize(x1 + 1, 0);
                }
                data[y][x1] += 1;
            }
        }
        if y1 == y2 {
            if data.len() < y1 + 1 {
                data.resize(y1 + 1, Vec::new());
            }
            let (x1, x2) = (cmp::min(x1, x2), cmp::max(x1, x2) + 1);
            if data[y1].len() < x2 {
                data[y1].resize(x2, 0);
            }
            for x in x1..x2 {
                data[y1][x] += 1;
            }
        }
    }
    let count = data
        .iter()
        .flat_map(|d| d.iter())
        .filter(|v| **v > 1)
        .count();

    println!("{}", count);
}
