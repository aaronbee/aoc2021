use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);

    let coord: (i32, i32) = reader
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| match l.split_once(" ") {
            Some((dir, val)) => match (dir, val.parse::<i32>().unwrap()) {
                ("forward", x) => (x, 0),
                ("down", y) => (0, y),
                ("up", y) => (0, -y),
                (other, _) => panic!("unhandled case: {}", other),
            },
            None => panic!("split failed"),
        })
        .fold((0, 0), |acc, (x, y)| (acc.0 + x, acc.1 + y));

    println!("{:?}: {}", coord, coord.0 * coord.1);
}
