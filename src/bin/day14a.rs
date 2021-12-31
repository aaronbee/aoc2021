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

    let iter = expand(&rules, template.bytes());
    let iter = expand(&rules, iter);
    let iter = expand(&rules, iter);
    let iter = expand(&rules, iter);
    let iter = expand(&rules, iter);
    let iter = expand(&rules, iter);
    let iter = expand(&rules, iter);
    let iter = expand(&rules, iter);
    let iter = expand(&rules, iter);
    let iter = expand(&rules, iter);

    let mut m: HashMap<u8, u64> = HashMap::new();
    for c in iter {
        let e = m.entry(c).or_insert(0);
        *e += 1;
    }
    let max = m.values().max().unwrap().to_owned();
    let min = m.values().min().unwrap().to_owned();
    println!("{:?}", max - min);
}

enum ExpandState {
    Empty(),
    EmitRule(u8),
    EmitB(u8),
}

struct Expand<'a, T>
where
    T: Iterator<Item = u8>,
{
    rules: &'a HashMap<(u8, u8), u8>,
    input: T,
    state: ExpandState,
}

impl<'a, T> Expand<'a, T>
where
    T: Iterator<Item = u8>,
{
    fn new(rules: &'a HashMap<(u8, u8), u8>, input: T) -> Self {
        Self {
            rules,
            input,
            state: ExpandState::Empty(),
        }
    }
}

impl<'a, T> Iterator for Expand<'a, T>
where
    T: Iterator<Item = u8>,
{
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        match self.state {
            ExpandState::Empty() => {
                let c = self.input.next();
                if let Some(a) = c {
                    self.state = ExpandState::EmitRule(a);
                }
                c
            }
            ExpandState::EmitRule(a) => {
                let c = self.input.next();
                if let Some(b) = c {
                    self.state = ExpandState::EmitB(b);
                    Some(*self.rules.get(&(a, b)).unwrap())
                } else {
                    self.state = ExpandState::Empty();
                    None
                }
            }
            ExpandState::EmitB(b) => {
                self.state = ExpandState::EmitRule(b);
                Some(b)
            }
        }
    }
}

fn expand<'a, T: Iterator<Item = u8>>(rules: &'a HashMap<(u8, u8), u8>, iter: T) -> Expand<'a, T> {
    Expand::new(rules, iter)
}
