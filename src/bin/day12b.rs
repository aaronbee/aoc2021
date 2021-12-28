use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Graph = HashMap<String, Vec<String>>;

fn is_lowercase(s: &String) -> bool {
    let c = s.as_bytes()[0];
    c >= b'a' && c <= b'z'
}

fn explore(g: &Graph, seen: &mut HashSet<String>, twice: bool, cave: &String) -> u64 {
    if cave == "end" {
        return 1;
    }
    if is_lowercase(&cave) {
        seen.insert(cave.clone());
    }
    let mut count = 0;
    for c in g.get(cave).unwrap().iter() {
        if is_lowercase(c) && seen.contains(c) {
            if !twice {
                count += explore(g, seen, true, c);
                seen.insert(c.clone());
            }
            continue;
        }
        count += explore(g, seen, twice, c);
    }
    if is_lowercase(cave) {
        seen.remove(cave);
    }
    return count;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);

    let mut g = Graph::new();

    for l in reader.lines().filter_map(|l| l.ok()) {
        let (mut lhs, mut rhs) = l.split_once('-').unwrap();
        if rhs == "start" || lhs == "end" {
            let tmp = rhs;
            rhs = lhs;
            lhs = tmp;
        }
        g.entry(lhs.to_owned())
            .or_insert(Vec::new())
            .push(rhs.to_owned());
        if lhs != "start" && rhs != "end" {
            g.entry(rhs.to_owned())
                .or_insert(Vec::new())
                .push(lhs.to_owned());
        }
    }

    let result = explore(&g, &mut HashSet::new(), false, &"start".to_owned());

    println!("{:?}", result);
}
