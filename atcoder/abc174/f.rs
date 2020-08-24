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
    ($next:expr, [graph1; $len:expr]) => {{
        let mut g = vec![vec![]; $len];
        let ab = read_value!($next, [(usize1, usize1)]);
        for (a, b) in ab {
            g[a].push(b);
            g[b].push(a);
        }
        g
    }};
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
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

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

fn bit_add(bit: &mut [i64], mut i: usize, x: i64) {
    while i < bit.len() {
        bit[i] += x;
        i += i & i.wrapping_neg();
    }
}

fn bit_acc(bit: &[i64], mut v: usize) -> i64 {
    let mut sum = 0;
    while v > 0 {
        sum += bit[v];
        v &= v - 1;
    }
    sum
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize, q: usize,
        c: [usize1; n],
        lr: [(usize1, usize); q],
    }
    let mut occ = vec![vec![]; n];
    for i in 0..n {
        occ[c[i]].push(i);
    }
    let mut pos = vec![0; n];
    let mut qs = vec![vec![]; n];
    for i in 0..q {
        let (l, r) = lr[i];
        qs[l].push((i, r));
    }
    let mut bit = vec![0; n + 1];
    let mut seen = vec![false; n];
    for i in 0..n {
        if !seen[c[i]] {
            bit_add(&mut bit, i + 1, 1);
            seen[c[i]] = true;
        }
    }
    let mut ans = vec![n as i64 + 1; q];
    for i in 0..n {
        for &(idx, r) in &qs[i] {
            // dp[r]
            ans[idx] = bit_acc(&bit, r);
        }
        let c = c[i];
        assert_eq!(occ[c][pos[c]], i);
        pos[c] += 1;
        if pos[c] < occ[c].len() {
            // for j in i..=pos[c] { dp[j] -= 1; }
            bit_add(&mut bit, occ[c][pos[c]] + 1, 1);
        }
        bit_add(&mut bit, i + 1, -1);
    }
    for i in 0..q {
        puts!("{}\n", ans[i]);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
