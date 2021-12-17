use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);

    let mut count = 0;
    for l in reader.lines().filter_map(|l| l.ok()) {
        let rhs = l.split_once(" | ").unwrap().1;
        count += rhs
            .split(' ')
            .filter(|s| match s.len() {
                2 | 4 | 3 | 7 => true,
                _ => false,
            })
            .count();
    }

    println!("{}", count);
}
