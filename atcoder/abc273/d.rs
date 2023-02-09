use std::cmp::*;
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes.by_ref().map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        h: i32, w: i32, rs: i32, cs: i32,
        n: usize,
        rc: [(i32, i32); n],
        q: usize,
        dl: [(char, i32); q],
    }
    let mut rows = BTreeMap::new();
    let mut cols = BTreeMap::new();
    for (r, c) in rc {
        rows.entry(r).or_insert(BTreeSet::new()).insert(c);
        cols.entry(c).or_insert(BTreeSet::new()).insert(r);
    }
    let mut x = rs;
    let mut y = cs;
    for (d, l) in dl {
        match d {
            'L' | 'R' => {
                if let Some(r) = rows.get(&x) {
                    if d == 'L' {
                        let nxt = if let Some(&nxt) = r.range(..=y).rev().next() {
                            nxt + 1
                        } else {
                            1
                        };
                        y = max(nxt, y - l);
                    } else {
                        let nxt = if let Some(&nxt) = r.range(y..).next() {
                            nxt - 1
                        } else {
                            w
                        };
                        y = min(nxt, y + l);
                    }
                } else {
                    y = if d == 'L' {
                        max(1, y - l)
                    } else {
                        min(w, y + l)
                    };
                }
            }
            'U' | 'D' => {
                if let Some(c) = cols.get(&y) {
                    if d == 'U' {
                        let nxt = if let Some(&nxt) = c.range(..=x).rev().next() {
                            nxt + 1
                        } else {
                            1
                        };
                        x = max(nxt, x - l);
                    } else {
                        let nxt = if let Some(&nxt) = c.range(x..).next() {
                            nxt - 1
                        } else {
                            h
                        };
                        x = min(nxt, x + l);
                    }
                } else {
                    x = if d == 'U' {
                        max(1, x - l)
                    } else {
                        min(h, x + l)
                    };
                }
            }
            _ => panic!(),
        }
        puts!("{} {}\n", x, y);
    }
}

