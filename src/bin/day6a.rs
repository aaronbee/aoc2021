use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);

    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();

    let mut v: Vec<u8> = buf
        .trim()
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect();

    const DAYS: u32 = 80;

    for i in 0..DAYS {
        println!("day {}: {}", i, v.len());
        let mut v2 = v.clone();
        for (i, v) in v.iter().enumerate() {
            if *v == 0 {
                v2[i] = 6;
                v2.push(8);
            } else {
                v2[i] -= 1;
            }
        }
        v = v2;
    }

    println!("{}", v.len());
}
