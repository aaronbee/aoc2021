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
        let (x1, y1) = (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap());
        let (x, y) = right.split_once(',').unwrap();
        let (x2, y2) = (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap());

        let max_x = cmp::max(x1, x2) + 1;
        let max_y = cmp::max(y1, y2) + 1;
        if data.len() < max_y as usize {
            data.resize(max_y as usize, Vec::new());
        }

        fn o2d(o: cmp::Ordering) -> i32 {
            match o {
                cmp::Ordering::Less => 1,
                cmp::Ordering::Equal => 0,
                cmp::Ordering::Greater => -1,
            }
        }
        let delta = (o2d(x1.cmp(&x2)), o2d(y1.cmp(&y2)));
        let (mut x, mut y) = (x1, y1);
        while (x, y) != (x2, y2) {
            if data[y as usize].len() < (max_x as usize) {
                data[y as usize].resize(max_x as usize, 0);
            }
            data[y as usize][x as usize] += 1;
            x += delta.0;
            y += delta.1;
        }
        if data[y as usize].len() < (max_x as usize) {
            data[y as usize].resize(max_x as usize, 0);
        }
        data[y as usize][x as usize] += 1;
    }
    let count = data
        .iter()
        .flat_map(|d| d.iter())
        .filter(|v| **v > 1)
        .count();

    println!("{}", count);
}
