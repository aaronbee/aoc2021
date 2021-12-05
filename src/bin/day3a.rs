use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let width = args[2].parse::<usize>().unwrap();
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);

    let mut data: Vec<Vec<bool>> = vec![Vec::new(); width];

    for l in reader.lines().filter_map(|l| l.ok()) {
        assert_eq!(l.len(), width);
        for (i, c) in l.char_indices() {
            data[i].push(c == '1')
        }
    }

    let mut gamma = 0;
    for i in 0..width {
        if data[i].iter().filter(|b| **b).count() > data[i].len() / 2 {
            gamma += 1;
        }
        gamma <<= 1;
    }
    gamma >>= 1;
    let epsilon = gamma ^ ((1 << width) - 1);

    println!("{} * {} = {}", gamma, epsilon, gamma * epsilon)
}
