use core::panic;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn score(brace: char) -> u32 {
    match brace {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("bad brace"),
    }
}

fn score_line(l: String) -> u32 {
    let mut stack = Vec::new();
    for c in l.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' => {
                if stack.pop().unwrap() != '(' {
                    return score(c);
                }
            }
            ']' => {
                if stack.pop().unwrap() != '[' {
                    return score(c);
                }
            }
            '}' => {
                if stack.pop().unwrap() != '{' {
                    return score(c);
                }
            }
            '>' => {
                if stack.pop().unwrap() != '<' {
                    return score(c);
                }
            }
            _ => panic!("unexpected char"),
        };
    }
    return 0;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);

    let score: u32 = reader.lines().filter_map(|l| l.ok()).map(score_line).sum();
    println!("{}", score);
}
