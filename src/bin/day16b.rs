use std::env;
use std::fs;

#[derive(PartialEq, Eq, Debug)]
enum Typ {
    Sum,     // 0
    Product, // 1
    Min,     // 2
    Max,     // 3
    Literal, // 4
    GT,      // 5
    LT,      // 6
    EQ,      // 7
}

#[derive(PartialEq, Eq, Debug)]
enum LengthTyp {
    Total,     // 0
    Subpacket, // 1
}

#[derive(Debug)]
struct BitsReader {
    v: Vec<u8>,
    i: usize,
}

impl BitsReader {
    fn from_str(s: &str) -> Self {
        let mut v: Vec<u8> = Vec::with_capacity(s.len());
        for i in 0..s.len() {
            v.push(u8::from_str_radix(&s[i..i + 1], 16).unwrap());
        }
        Self { v, i: 0 }
    }

    fn read(&mut self, count: usize) -> u64 {
        let mut i = self.i;
        let b = i / 4;
        let o = i % 4;
        let mask = (1 << (4 - o)) - 1;
        let mut result: u64 = self.v[b] as u64 & mask;
        i += 4 - o;
        while i - self.i < count {
            assert_eq!(i % 4, 0);
            let b = i / 4;
            result = result << 4;
            result = result | self.v[b] as u64;
            i += 4;
        }
        result = result >> (i - self.i) - count;
        self.i += count;
        result
    }

    fn version(&mut self) -> u64 {
        self.read(3)
    }

    fn typ(&mut self) -> Typ {
        match self.read(3) {
            0 => Typ::Sum,
            1 => Typ::Product,
            2 => Typ::Min,
            3 => Typ::Max,
            4 => Typ::Literal,
            5 => Typ::GT,
            6 => Typ::LT,
            7 => Typ::EQ,
            _ => panic!("bad type"),
        }
    }

    fn literal(&mut self) -> u64 {
        let mut more = self.read(1) == 1;
        let mut result = self.read(4);
        while more {
            more = self.read(1) == 1;
            result = (result << 4) | self.read(4);
        }
        result
    }

    fn length_typ(&mut self) -> LengthTyp {
        match self.read(1) {
            0 => LengthTyp::Total,
            _ => LengthTyp::Subpacket,
        }
    }

    fn total_len(&mut self) -> usize {
        self.read(15) as usize
    }

    fn subpacket_count(&mut self) -> u64 {
        self.read(11)
    }

    fn index(&self) -> usize {
        self.i
    }
}

fn eval(mut b: BitsReader) -> u64 {
    eval_packet(&mut b)
}

fn eval_packet(b: &mut BitsReader) -> u64 {
    b.version();
    let t = b.typ();
    if t == Typ::Literal {
        return b.literal();
    }
    let v = eval_subpackets(b);
    match t {
        Typ::Literal => panic!("can't happen"),
        Typ::Sum => v.iter().sum(),
        Typ::Product => v.iter().product(),
        Typ::Min => *v.iter().min().unwrap(),
        Typ::Max => *v.iter().max().unwrap(),
        Typ::GT => {
            assert_eq!(v.len(), 2);
            if v[0] > v[1] {
                1
            } else {
                0
            }
        }
        Typ::LT => {
            assert_eq!(v.len(), 2);
            if v[0] < v[1] {
                1
            } else {
                0
            }
        }
        Typ::EQ => {
            assert_eq!(v.len(), 2);
            if v[0] == v[1] {
                1
            } else {
                0
            }
        }
    }
}

fn eval_subpackets(b: &mut BitsReader) -> Vec<u64> {
    let mut v = Vec::new();
    match b.length_typ() {
        LengthTyp::Total => {
            let t = b.total_len();
            let i = b.index();
            while b.index() < i + t {
                v.push(eval_packet(b));
            }
        }
        LengthTyp::Subpacket => {
            for _ in 0..b.subpacket_count() {
                v.push(eval_packet(b));
            }
        }
    }
    v
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let s = fs::read_to_string(filename).unwrap();

    let b = BitsReader::from_str(&s);

    println!("{:?}", eval(b));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let mut b = BitsReader::from_str("8A");
        assert_eq!(b.read(3), 4);
        assert_eq!(b.read(3), 2);
        let mut b = BitsReader::from_str("9A");
        assert_eq!(b.read(3), 4);
        assert_eq!(b.read(3), 6);
        let mut b = BitsReader::from_str("8A");
        assert_eq!(b.read(3), 4);
        assert_eq!(b.read(5), 10);
        let mut b = BitsReader::from_str("9A");
        assert_eq!(b.read(3), 4);
        assert_eq!(b.read(5), 26);
    }

    #[test]
    fn test_eval() {
        let b = BitsReader::from_str("C200B40A82");
        assert_eq!(eval(b), 3);
        let b = BitsReader::from_str("04005AC33890");
        assert_eq!(eval(b), 54);
        let b = BitsReader::from_str("CE00C43D881120");
        assert_eq!(eval(b), 9);
        let b = BitsReader::from_str("D8005AC2A8F0");
        assert_eq!(eval(b), 1);
        let b = BitsReader::from_str("F600BC2D8F");
        assert_eq!(eval(b), 0);
        let b = BitsReader::from_str("9C005AC2F8F0");
        assert_eq!(eval(b), 0);
        let b = BitsReader::from_str("9C0141080250320F1802104A08");
        assert_eq!(eval(b), 1);
    }
}
