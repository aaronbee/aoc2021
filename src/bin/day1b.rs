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

    let iter1 = numbers.iter();
    let iter2 = numbers[1..].iter();
    let iter3 = numbers[2..].iter();
    let triples: Vec<i32> = iter1
        .zip(iter2)
        .zip(iter3)
        .map(|((x, y), z)| x + y + z)
        .collect();

    let iter1 = triples.iter();
    let iter2 = triples[1..].iter();
    let sum: i32 = iter1
        .zip(iter2)
        .map(|(x, y)| if y > x { 1 } else { 0 })
        .sum();
    println!("{}", sum);
}
