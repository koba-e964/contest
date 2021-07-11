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

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn find_basis(a: &[Vec<i32>]) -> Vec<Vec<i32>> {
    if a.is_empty() {
        return vec![];
    }
    let n = a.len();
    let m = a[0].len();
    let mut basis: Vec<Vec<i32>> = vec![];
    for i in 0..n {
        let mut cur = a[i].clone();
        for b in &basis {
            let mut xor = cur.clone();
            for i in 0..m {
                xor[i] ^= b[i];
            }
            if xor < cur {
                cur = xor;
            }
        }
        if cur.iter().any(|&x| x != 0) {
            basis.push(cur);
        }
    }
    basis
}

// Tags: gauss-elimination
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize,
        a: [[usize1]; n],
        s: [i32; m],
    }
    let mut mat = vec![vec![0; m]; n];
    for i in 0..n {
        for &a in &a[i] {
            mat[i][a] = 1;
        }
    }
    let basis = find_basis(&mat);
    let mut s = s;
    for b in &basis {
        let mut xor = s.clone();
        for i in 0..m {
            xor[i] ^= b[i];
        }
        s.chmin(xor);
    }
    if s.iter().any(|&x| x != 0) {
        puts!("0\n");
        return;
    }
    let dimker = n - basis.len();
    let mut ans = 1;
    for _ in 0..dimker {
        ans = ans * 2 % 998_244_353;
    }
    puts!("{}\n", ans);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
