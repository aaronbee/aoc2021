use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn flash(v: &mut Vec<Vec<u8>>, s: &mut HashSet<(usize, usize)>, x: usize, y: usize) {
    s.insert((x, y));
    for (dx, dy) in vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ] {
        let xx = x as i32 + dx;
        let yy = y as i32 + dy;
        if yy < 0 || yy as usize >= v.len() {
            continue;
        }
        let yy = yy as usize;
        if xx < 0 || xx as usize >= v[yy].len() {
            continue;
        }
        let xx = xx as usize;
        v[yy][xx] += 1;
        if v[yy][xx] >= 10 && !s.contains(&(xx, yy)) {
            flash(v, s, xx, yy);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);

    let mut v: Vec<Vec<u8>> = reader
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| l.bytes().map(|c| c - '0' as u8).collect::<Vec<u8>>())
        .collect();

    const STEPS: u32 = 100;
    let mut count: u64 = 0;
    let mut s = HashSet::new();
    for _ in 0..STEPS {
        for y in 0..v.len() {
            for x in 0..v[y].len() {
                v[y][x] += 1;
            }
        }
        for y in 0..v.len() {
            for x in 0..v[y].len() {
                if v[y][x] >= 10 && !s.contains(&(x, y)) {
                    flash(&mut v, &mut s, x, y);
                }
            }
        }
        for y in 0..v.len() {
            for x in 0..v[y].len() {
                if v[y][x] >= 10 {
                    if !s.contains(&(x, y)) {
                        panic!("set missing ({}, {})", x, y);
                    }
                    count += 1;
                    v[y][x] = 0;
                }
            }
        }
        s.clear();
    }

    println!("{}", count);
}
