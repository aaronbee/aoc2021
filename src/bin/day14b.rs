use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);

    let mut iter = reader.lines();
    let template = iter.next().unwrap().unwrap();
    iter.next();

    let mut rules = HashMap::new();
    while let Some(l) = iter.next() {
        let l = l.unwrap();
        let (lhs, rhs) = l.split_once(" -> ").unwrap();
        let pair = match lhs.as_bytes() {
            [a, b] => (*a, *b),
            _ => panic!("unexpected lhs"),
        };
        let insert = rhs.as_bytes()[0];
        rules.insert(pair, insert);
    }
    let rules = rules;

    let mut pairs: HashMap<(u8, u8), u64> = HashMap::new();
    for pair in template.as_bytes().windows(2) {
        match pair {
            [a, b] => *pairs.entry((*a, *b)).or_insert(0) += 1,
            _ => panic!("huh?"),
        }
    }

    for _ in 0..40 {
        let mut new_pairs = HashMap::with_capacity(pairs.len());
        for (pair, count) in pairs.iter() {
            let b = *rules.get(pair).unwrap();
            let (a, c) = *pair;
            *new_pairs.entry((a, b)).or_insert(0) += *count;
            *new_pairs.entry((b, c)).or_insert(0) += *count;
        }
        pairs = new_pairs;
    }

    let mut m: HashMap<u8, u64> = HashMap::new();
    m.insert(template.bytes().last().unwrap(), 1);
    for (pair, count) in pairs.iter() {
        let (a, _b) = *pair;
        *m.entry(a).or_insert(0) += count;
    }
    let max = m.values().max().unwrap().to_owned();
    let min = m.values().min().unwrap().to_owned();
    println!("{:?}", max - min);
}
