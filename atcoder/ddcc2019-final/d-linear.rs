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
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

extern crate core;
use core::num::Wrapping;
const SZ: usize = 6;

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        s: chars,
        lr: [(usize1, usize)],
    }
    let n = s.len();
    let mut unit = [[Wrapping(0u32); SZ]; SZ];
    for i in 0 .. SZ { unit[i][i] = Wrapping(1); }
    let mut acc = vec![[[Wrapping(0u32); SZ]; SZ]; n + 1];
    let mut accinv = vec![[[Wrapping(0u32); SZ]; SZ]; n + 1];
    acc[0] = unit;
    accinv[0] = unit;
    for i in 0 .. n {
        let idx = match s[i] {
            'D' => 0,
            'I' => 1,
            'S' => 2,
            'C' => 3,
            'O' => 4,
            _ => unreachable!(),
        };
        acc[i + 1] = acc[i];
        for j in 0 .. SZ { acc[i + 1][j][idx + 1] += acc[i + 1][j][idx]; }
        accinv[i + 1] = accinv[i];
        for j in 0 .. SZ { accinv[i + 1][idx][j] -= accinv[i + 1][idx + 1][j]; }
    }
    for (l, r) in lr {
        let mut ans = Wrapping(0);
        for i in 0 .. SZ { ans += accinv[l][0][i] * acc[r][i][5]; }
        puts!("{}\n", ans);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
