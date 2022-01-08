use std::env;
use std::fs;

#[derive(PartialEq, Eq, Debug)]
enum Typ {
    Literal,  // 4
    Operator, // !4
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
            4 => Typ::Literal,
            _ => Typ::Operator,
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

fn version_sum(mut b: BitsReader) -> u64 {
    version_sum_packet(&mut b)
}

fn version_sum_packet(b: &mut BitsReader) -> u64 {
    let mut sum = b.version();
    match b.typ() {
        Typ::Literal => {
            b.literal();
        }
        Typ::Operator => match b.length_typ() {
            LengthTyp::Total => {
                let t = b.total_len();
                let i = b.index();
                while b.index() < i + t {
                    sum += version_sum_packet(b);
                }
            }
            LengthTyp::Subpacket => {
                for _ in 0..b.subpacket_count() {
                    sum += version_sum_packet(b);
                }
            }
        },
    }
    sum
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let s = fs::read_to_string(filename).unwrap();

    let b = BitsReader::from_str(&s);

    println!("{:?}", version_sum(b));
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
    fn test_bitsreader() {
        let mut b = BitsReader::from_str("EE00D40C823060");
        assert_eq!(b.version(), 7);
        assert_eq!(b.typ(), Typ::Operator);
        assert_eq!(b.length_typ(), LengthTyp::Subpacket);
        assert_eq!(b.subpacket_count(), 3);
        b.version();
        assert_eq!(b.typ(), Typ::Literal);
        assert_eq!(b.literal(), 1);
        b.version();
        assert_eq!(b.typ(), Typ::Literal);
        assert_eq!(b.literal(), 2);
        b.version();
        assert_eq!(b.typ(), Typ::Literal);
        assert_eq!(b.literal(), 3);
    }

    #[test]
    fn test_version_sum() {
        let b = BitsReader::from_str("8A004A801A8002F478");
        assert_eq!(version_sum(b), 16);
        let b = BitsReader::from_str("620080001611562C8802118E34");
        assert_eq!(version_sum(b), 12);
        let b = BitsReader::from_str("C0015000016115A2E0802F182340");
        assert_eq!(version_sum(b), 23);
        let b = BitsReader::from_str("A0016C880162017C3686B18A3D4780");
        assert_eq!(version_sum(b), 31);
    }
}
