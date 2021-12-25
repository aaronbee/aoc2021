use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

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

    let mut risk: u32 = 0;
    for y in 0..v.len() {
        for x in 0..v[y].len() {
            let c = v[y][x];
            if x > 0 && v[y][x - 1] <= c {
                continue;
            }
            if x + 1 < v[y].len() && v[y][x + 1] <= c {
                continue;
            }
            if y > 0 && v[y - 1][x] <= c {
                continue;
            }
            if y + 1 < v.len() && v[y + 1][x] <= c {
                continue;
            }
            risk += c as u32 + 1;
        }
    }
    println!("{}", risk)
}
