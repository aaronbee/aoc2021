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

    let v: Vec<u8> = buf
        .trim()
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect();

    const DAYS: usize = 256;

    let mut table = [[0 as u64; 9]; DAYS + 1];

    for i in v {
        table[0][i as usize] += 1;
    }

    for i in 1..DAYS + 1 {
        for j in 0..8 {
            table[i][j] = table[i - 1][j + 1];
        }
        table[i][6] += table[i - 1][0];
        table[i][8] = table[i - 1][0];
    }

    println!("{}", table[DAYS].iter().sum::<u64>());
}
