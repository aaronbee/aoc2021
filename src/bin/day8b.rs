use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn letters2num(s: &str) -> u8 {
    let mut n: u8 = 0;
    for c in s.bytes() {
        n |= 1 << c - 'a' as u8;
    }
    return n;
}

fn solve(patterns: Vec<u8>, outputs: Vec<u8>) -> u32 {
    let mut len2: u8 = 0;
    let mut len4: u8 = 0;
    let mut len5: Vec<u8> = Vec::with_capacity(3);
    let mut len6: Vec<u8> = Vec::with_capacity(3);
    let mut m: HashMap<u8, u8> = HashMap::new();
    for p in patterns.iter() {
        match p.count_ones() {
            2 => {
                len2 = *p;
                m.insert(len2, 1);
            }
            3 => {
                m.insert(*p, 7);
            }
            4 => {
                len4 = *p;
                m.insert(len4, 4);
            }
            5 => {
                len5.push(*p);
            }
            6 => {
                len6.push(*p);
            }
            7 => {
                m.insert(*p, 8);
            }
            _ => panic!("unexpected number"),
        };
    }
    assert_eq!(len5.len(), 3); // 2, 3, 5
    assert_eq!(len6.len(), 3); // 0, 6, 9

    // extra letters in 4 len compared to 2 len must be the set {b, d}
    let bd = len4 ^ len2;
    assert_eq!(bd.count_ones(), 2);
    // 5 will contain b & d
    let pos = len5.iter().position(|n| n & bd == bd).unwrap();
    let five = len5.swap_remove(pos);
    m.insert(five, 5);
    // 2,3 contain d, but not b
    let d = len5[0] & bd;
    assert_eq!(d.count_ones(), 1);
    // The difference between 2,3 will be {e, f}
    let ef = len5[0] ^ len5[1];
    assert_eq!(ef.count_ones(), 2);
    // 9 won't contain both {e, f}
    let pos = len6.iter().position(|n| n & ef != ef).unwrap();
    let nine = len6.swap_remove(pos);
    m.insert(nine, 9);
    let f = nine & ef;
    // 3 will contain f
    let pos = len5.iter().position(|n| n & f == f).unwrap();
    let three = len5.swap_remove(pos);
    m.insert(three, 3);
    m.insert(len5[0], 2);
    // 6 will contain d
    let pos = len6.iter().position(|n| n & d == d).unwrap();
    let six = len6.swap_remove(pos);
    m.insert(six, 6);
    m.insert(len6[0], 0);

    let mut result: u32 = 0;
    for p in outputs.iter() {
        result += u32::from(*m.get(p).unwrap());
        result *= 10;
    }
    result /= 10;
    return result;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);

    let mut sum: u32 = 0;
    for l in reader.lines().filter_map(|l| l.ok()) {
        let line = l.split_once(" | ").unwrap();
        let patterns: Vec<u8> = line.0.split(' ').map(letters2num).collect();
        let outputs: Vec<u8> = line.1.split(' ').map(letters2num).collect();
        sum += solve(patterns, outputs);
    }

    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testletters2num() {
        assert_eq!(letters2num("abcdefg"), 0b01111111);
        assert_eq!(letters2num("gbcef"), 0b01110110);
    }

    #[test]
    fn test_solve() {
        assert_eq!(
            solve(
                "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb"
                    .split(' ')
                    .map(letters2num)
                    .collect(),
                "be be be be".split(' ').map(letters2num).collect(),
            ),
            1111
        );
        assert_eq!(
            solve(
                "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb"
                    .split(' ')
                    .map(letters2num)
                    .collect(),
                "fdgacbe cefdb cefbgd gcbe"
                    .split(' ')
                    .map(letters2num)
                    .collect(),
            ),
            8394
        );
    }
}
