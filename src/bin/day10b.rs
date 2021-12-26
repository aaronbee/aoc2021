use core::panic;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn score(brace: char) -> u32 {
    match brace {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("bad brace"),
    }
}

fn score_line(l: String) -> Option<u64> {
    let mut stack = Vec::new();
    for c in l.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' => {
                if stack.pop().unwrap() != '(' {
                    return None;
                }
            }
            ']' => {
                if stack.pop().unwrap() != '[' {
                    return None;
                }
            }
            '}' => {
                if stack.pop().unwrap() != '{' {
                    return None;
                }
            }
            '>' => {
                if stack.pop().unwrap() != '<' {
                    return None;
                }
            }
            _ => panic!("unexpected char"),
        };
    }
    let s = stack
        .iter()
        .rev()
        .fold(0, |accum: u64, x| accum * 5 + score(*x) as u64);
    return Some(s);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);

    let mut scores: Vec<u64> = reader
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(score_line)
        .collect();
    scores.sort();

    println!("{}", scores[scores.len() / 2]);
}
