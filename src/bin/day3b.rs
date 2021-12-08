use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Seek;
use std::io::SeekFrom;

fn filter<F: Fn(usize, usize) -> u32>(width: usize, mut v: Vec<u32>, filt: F) -> u32 {
    for i in (0..width).rev() {
        if v.len() == 1 {
            break;
        }
        let mask = 1 << i;
        let ones = v.iter().filter(|n| **n & mask == mask).count();
        let filt = filt(ones, v.len()) << i;
        v.retain(|n| n & mask == filt);
    }
    v[0]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();
    let width = buf.trim().len();
    reader.seek(SeekFrom::Start(0)).unwrap();

    let numbers: Vec<u32> = reader
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| u32::from_str_radix(&l, 2).unwrap())
        .collect();

    let oxygen = filter(
        width,
        numbers.clone(),
        |ones, len| if ones * 2 >= len { 1 } else { 0 },
    );
    let co2 = filter(
        width,
        numbers.clone(),
        |ones, len| if ones * 2 < len { 1 } else { 0 },
    );
    println!("{} * {} = {}", oxygen, co2, oxygen * co2);
}
