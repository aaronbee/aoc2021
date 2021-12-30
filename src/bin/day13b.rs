use bit_vec::BitVec;
use core::panic;
use std::cmp::max;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);

    let mut coords: Vec<(usize, usize)> = Vec::new();
    let mut folds: Vec<(u8, usize)> = Vec::new();
    let mut iter = reader.lines().filter_map(|l| l.ok());
    loop {
        let l = iter.next().unwrap();
        if l.len() == 0 {
            break;
        }
        let (x, y) = l.split_once(',').unwrap();
        coords.push((x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()));
    }
    loop {
        let l = match iter.next() {
            Some(l) => l,
            None => break,
        };
        let l = l.trim_start_matches("fold along ");
        let (lhs, rhs) = l.split_once('=').unwrap();
        assert_eq!(lhs.len(), 1);
        let axis = lhs.as_bytes()[0];
        let n = rhs.parse::<usize>().unwrap();
        folds.push((axis, n));
    }

    let (max_x, max_y) = coords.iter().fold((0, 0), |(max_x, max_y), (x, y)| {
        (max(max_x, *x), max(max_y, *y))
    });
    let max_x = max_x + 1;
    let max_y = max_y + 1;

    let mut v = vec![BitVec::from_elem(max_x, false); max_y];

    for (x, y) in &coords {
        v[*y].set(*x, true);
    }

    for (f, i) in folds.iter() {
        match *f {
            b'x' => fold_ver(&mut v, *i),
            b'y' => fold_hor(&mut v, *i),
            _ => panic!("huh?"),
        }
    }

    for bv in v {
        let s: String = bv.iter().map(|b| if b { '#' } else { '.' }).collect();
        println!("{}", s);
    }
}

fn fold_hor(v: &mut Vec<BitVec>, i: usize) {
    assert_eq!(v.len() / 2, i);
    let u = v.split_off(i);
    assert_eq!(v.len(), u.len() - 1);
    for (bv, l) in v.iter_mut().zip(u.iter().rev()) {
        bv.or(l);
    }
    v.truncate(i);
}

fn fold_ver(v: &mut Vec<BitVec>, i: usize) {
    assert_eq!(v[0].len() / 2, i);
    for bv in v.iter_mut() {
        let r = bv.split_off(i);
        let r: BitVec = r.iter().skip(1).rev().collect();
        bv.or(&r);
    }
}
