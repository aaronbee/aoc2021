use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

enum Move {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);

    let coord: (i32, i32, i32) = reader
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| match l.split_once(" ") {
            Some((dir, val)) => match (dir, val.parse::<i32>().unwrap()) {
                ("forward", x) => Move::Forward(x),
                ("down", x) => Move::Down(x),
                ("up", x) => Move::Up(x),
                (other, _) => panic!("unhandled case: {}", other),
            },
            None => panic!("split failed"),
        })
        .fold((0, 0, 0), |acc, mov| match mov {
            Move::Forward(x) => (acc.0 + x, acc.1 + acc.2 * x, acc.2),
            Move::Down(x) => (acc.0, acc.1, acc.2 + x),
            Move::Up(x) => (acc.0, acc.1, acc.2 - x),
        });

    println!("{:?}: {}", coord, coord.0 * coord.1);
}
