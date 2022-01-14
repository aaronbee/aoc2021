use std::cell::RefCell;
use std::cmp::max;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Clone)]
struct Node {
    e: RefCell<E>,
}

#[derive(Clone)]
enum E {
    Value(i32),
    Pair(Box<Node>, Box<Node>),
}

impl E {
    fn new_pair(lv: i32, rv: i32) -> Self {
        E::Pair(
            Box::new(Node {
                e: RefCell::new(E::Value(lv)),
            }),
            Box::new(Node {
                e: RefCell::new(E::Value(rv)),
            }),
        )
    }
}

impl Node {
    fn parse<I: Iterator<Item = u8>>(i: &mut I) -> Self {
        let b = i.next().unwrap();
        if b == b'[' {
            let l = Node::parse(i);
            assert_eq!(i.next().unwrap(), b',');
            let r = Node::parse(i);
            assert_eq!(i.next().unwrap(), b']');
            Node {
                e: RefCell::new(E::Pair(Box::new(l), Box::new(r))),
            }
        } else {
            let v = b - b'0';
            Node {
                e: RefCell::new(E::Value(v as i32)),
            }
        }
    }

    fn explode(&self, depth: i32) -> (bool, Option<i32>, Option<i32>) {
        match *self.e.borrow() {
            E::Value(_) => (false, None, None),
            E::Pair(ref l, ref r) => {
                if depth < 3 {
                    match l.explode(depth + 1) {
                        (true, lv, Some(rv)) => {
                            r.add_to_left(rv);
                            return (true, lv, None);
                        }
                        (true, l, None) => return (true, l, None),
                        (false, _, _) => (),
                    }
                    return match r.explode(depth + 1) {
                        (true, Some(lv), rv) => {
                            l.add_to_right(lv);
                            (true, None, rv)
                        }
                        (true, None, r) => (true, None, r),
                        (false, _, _) => (false, None, None),
                    };
                } else {
                    let (is_pair, lc, rc) = l.to_tuple();
                    if is_pair {
                        l.e.replace(E::Value(0));
                        r.add_to_left(rc);
                        return (is_pair, Some(lc), None);
                    }
                    let (is_pair, lc, rc) = r.to_tuple();
                    if is_pair {
                        r.e.replace(E::Value(0));
                        l.add_to_right(lc);
                        return (is_pair, None, Some(rc));
                    }
                    (false, None, None)
                }
            }
        }
    }

    fn to_tuple(&self) -> (bool, i32, i32) {
        match *self.e.borrow() {
            E::Value(_) => (false, 0, 0),
            E::Pair(ref l, ref r) => {
                let lv: i32;
                let rv: i32;
                if let E::Value(v) = *l.e.borrow() {
                    lv = v;
                } else {
                    panic!("bad input");
                }
                if let E::Value(v) = *r.e.borrow() {
                    rv = v;
                } else {
                    panic!("bad input");
                }
                (true, lv, rv)
            }
        }
    }

    fn add_to_right(&self, x: i32) {
        let v: i32;
        match *self.e.borrow() {
            E::Value(y) => {
                v = x + y;
            }
            E::Pair(_, ref r) => {
                r.add_to_right(x);
                return;
            }
        }
        self.e.replace(E::Value(v));
    }

    fn add_to_left(&self, x: i32) {
        let v: i32;
        match *self.e.borrow() {
            E::Value(y) => {
                v = x + y;
            }
            E::Pair(ref l, _) => {
                l.add_to_left(x);
                return;
            }
        }
        self.e.replace(E::Value(v));
    }

    fn split(&self) -> bool {
        let v: i32;
        match *self.e.borrow() {
            E::Value(x) => {
                v = x;
            }
            E::Pair(ref l, ref r) => {
                if l.split() {
                    return true;
                }
                return r.split();
            }
        }
        if v < 10 {
            return false;
        }
        let lv = v / 2;
        let rv = v / 2 + (v % 2);
        self.e.replace(E::new_pair(lv, rv));
        true
    }

    fn add(self, other: Node) -> Node {
        Node {
            e: RefCell::new(E::Pair(Box::new(self), Box::new(other))),
        }
    }

    fn reduce(&self) {
        loop {
            while let (true, _, _) = self.explode(0) {}
            if !self.split() {
                return;
            }
        }
    }

    fn magnitude(&self) -> i32 {
        match *self.e.borrow() {
            E::Value(v) => v,
            E::Pair(ref l, ref r) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&*self.e.borrow(), f)
    }
}

impl fmt::Display for E {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            E::Value(v) => write!(f, "{}", *v),
            E::Pair(l, r) => {
                write!(f, "[")?;
                std::fmt::Display::fmt(l, f)?;
                write!(f, ",")?;
                std::fmt::Display::fmt(r, f)?;
                write!(f, "]")
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);

    let trees: Vec<Node> = reader
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| Node::parse(&mut l.bytes()))
        .collect();

    let mut max_magnitude: i32 = 0;
    for i in 0..trees.len() {
        for j in 0..trees.len() {
            if i == j {
                continue;
            }
            let t = trees[i].clone().add(trees[j].clone());
            t.reduce();
            max_magnitude = max(max_magnitude, t.magnitude());
        }
    }
    println!("{}", max_magnitude);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explode() {
        for (input, expected) in vec![
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
        ] {
            let n = Node::parse(&mut input.bytes());
            n.explode(0);
            assert_eq!(n.to_string(), expected);
        }
    }

    #[test]
    fn test_split() {
        let n = Node::parse(&mut "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]".bytes());
        n.explode(0);
        assert_eq!(n.to_string(), "[[[[0,7],4],[15,[0,13]]],[1,1]]");
        n.split();
        assert_eq!(n.to_string(), "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
        n.split();
        assert_eq!(n.to_string(), "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");
    }
}
