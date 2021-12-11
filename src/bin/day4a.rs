use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

#[derive(Debug)]
struct Board {
    a: [[(u32, bool); 5]; 5],
}

fn read_board<R: Read>(b: &mut BufReader<R>) -> Option<Board> {
    let mut board = Board {
        a: [[(0, false); 5]; 5],
    };
    let mut buf = String::new();
    for i in 0..5 {
        match b.read_line(&mut buf) {
            Err(_) | Ok(0) => return None,
            _ => (),
        }

        board.a[i] = buf
            .trim()
            .split_whitespace()
            .map(|num| (num.parse::<u32>().unwrap(), false))
            .collect::<Vec<(u32, bool)>>()
            .try_into()
            .unwrap();
        buf.clear();
    }
    Some(board)
}

fn mark(b: &mut Board, n: u32) -> bool {
    for i in 0..5 {
        for j in 0..5 {
            if b.a[i][j].0 == n {
                b.a[i][j].1 = true;
                return true;
            }
        }
    }
    return false;
}

fn check(b: &Board) -> bool {
    for i in 0..5 {
        if b.a[i].iter().all(|(_, v)| *v) {
            return true;
        }
    }
    for i in 0..5 {
        if b.a.iter().all(|a| a[i].1) {
            return true;
        }
    }
    return false;
}

fn score(b: &Board, i: u32) -> u32 {
    b.a.iter()
        .flat_map(|a| a.iter())
        .filter(|(_v, b)| !b)
        .map(|(v, _b)| *v)
        .sum::<u32>()
        * i
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();

    let numbers: Vec<u32> = buf
        .trim()
        .split(',')
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    buf.clear();

    let mut boards = Vec::<Board>::new();
    while let Ok(s) = reader.read_line(&mut buf) {
        if s == 0 {
            break;
        }
        buf.clear();
        if let Some(board) = read_board(&mut reader) {
            boards.push(board);
        } else {
            break;
        }
    }

    for i in numbers {
        for b in &mut boards {
            if mark(b, i) {
                if check(&b) {
                    println!("winner! {}", score(b, i));
                    return;
                }
            }
        }
    }
}
