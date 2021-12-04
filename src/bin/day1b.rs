use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);

    let numbers: Vec<i32> = reader
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| l.parse::<i32>().unwrap())
        .collect();

    let triples: Vec<i32> = numbers
        .iter()
        .zip(numbers[1..].iter())
        .zip(numbers[2..].iter())
        .map(|((x, y), z)| x + y + z)
        .collect();

    let sum: i32 = triples
        .iter()
        .zip(triples[1..].iter())
        .map(|(x, y)| if y > x { 1 } else { 0 })
        .sum();
    println!("{}", sum);
}
