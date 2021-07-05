#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Tags: permutation, construction
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        p: [usize1; n],
    }
    let mut dec = 0;
    let mut p = p;
    let mut inv = vec![0; n];
    for i in 0..n {
        inv[p[i]] = i;
    }
    let mut ops = vec![];
    for i in 0..n {
        let idx = inv[i];
        if idx == i {
            continue;
        }
        if i != dec {
            puts!("-1\n");
            return;
        }
        assert!(i < idx);
        for j in (i..idx).rev() {
            p.swap(j, j + 1);
            inv.swap(p[j], p[j + 1]);
            ops.push(j);
        }
        dec = idx;
    }
    if ops.len() != n - 1 {
        puts!("-1\n");
        return;
    }
    assert_eq!(ops.len(), n - 1);
    for i in 0..n - 1 {
        puts!("{}\n", ops[i] + 1);
    }
}
