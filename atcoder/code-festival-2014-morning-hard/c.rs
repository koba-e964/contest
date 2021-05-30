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

fn add(bit: &mut [Vec<i64>], a: usize, b: usize, val: i64) {
    let mut a = a + 1;
    let n = bit.len();
    let m = bit[0].len();
    while a < n {
        let mut b = b + 1;
        while b < m {
            bit[a][b] += val;
            b += b & b.wrapping_neg();
        }
        a += a & a.wrapping_neg();
    }
}

fn acc(bit: &[Vec<i64>], mut a: usize, b: usize) -> i64 {
    let mut sum = 0;
    while a > 0 {
        let mut b = b;
        while b > 0 {
            sum += bit[a][b];
            b &= b - 1;
        }
        a &= a - 1;
    }
    sum
}

// Tags: slower-than-intended-solutions
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize, k: usize,
        a: [[usize1; m]; n],
        q: usize,
        txy: [(i32, usize1, usize1, usize1, usize1); q],
    }
    let mut bit = vec![vec![vec![0i64; m + 1]; n + 1]; k];
    for i in 0..n {
        for j in 0..m {
            add(&mut bit[a[i][j]], i, j, 1);
        }
    }
    let mut a = a;
    for (t, x1, y1, x2, y2) in txy {
        if t == 1 {
            // exchange
            let c = a[x1][y1];
            let d = a[x2][y2];
            add(&mut bit[c], x1, y1, -1);
            add(&mut bit[d], x2, y2, -1);
            add(&mut bit[d], x1, y1, 1);
            add(&mut bit[c], x2, y2, 1);
            a[x1][y1] = d;
            a[x2][y2] = c;
        } else {
            // query
            let mut ma = (0, 0);
            for i in 0..k {
                let mut val = acc(&mut bit[i], x2 + 1, y2 + 1);
                val -= acc(&mut bit[i], x2 + 1, y1);
                val -= acc(&mut bit[i], x1, y2 + 1);
                val += acc(&mut bit[i], x1, y1);
                ma.chmax((val, i));
            }
            puts!("{} {}\n", ma.1 + 1, ma.0);
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
