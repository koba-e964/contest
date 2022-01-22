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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn calc(s: &[Vec<char>]) -> bool {
    if s.iter().any(|s| {
        let mut t = s.to_vec();
        t.reverse();
        s == &t
    }) {
        return true;
    }
    let mut seen = HashSet::new();
    let mut seen2 = HashSet::new();
    for v in s {
        if seen.contains(v) {
            return true;
        }
        if v.len() == 3 && seen.contains(&v[1..]) {
            return true;
        }
        if v.len() == 2 && seen2.contains(v) {
            return true;
        }
        let mut v = v.to_vec();
        v.reverse();
        seen.insert(v.clone());
        if v.len() == 3 {
            seen2.insert(v[1..].to_vec());
        }
    }
    false
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input!(s: [[chars]]);
    for s in s {
        puts!("{}\n", if calc(&s) { "YES" } else { "NO" });
    }
}
