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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn quo(a: i64, b: i64) -> i64 {
    assert!(b > 0);
    let mut r = a % b;
    if r < 0 {
        r += b;
    }
    (a - r) / b
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize,
        a: [i64; n],
    }
    let mut set = vec![BTreeSet::new(); m];
    let mm = m as i64;
    for i in 0..n {
        let dif = i as i64 + 1;
        let lo = max(1, quo(-a[i] + dif - 1, dif));
        let hi = min(mm + 1, quo(mm - a[i] + dif - 1, dif));
        for x in lo..hi {
            set[x as usize - 1].insert(a[i] + x * dif);
        }
    }
    for i in 0..m {
        let mut mex = 0;
        while set[i].contains(&mex) {
            mex += 1;
        }
        puts!("{}\n", mex);
    }
}
