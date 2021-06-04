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

trait Bisect<T> {
    fn lower_bound(&self, val: &T) -> usize;
    fn upper_bound(&self, val: &T) -> usize;
}

impl<T: Ord> Bisect<T> for [T] {
    fn lower_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] >= val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
    fn upper_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] > val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
}

// Tags: sqrt-decomposition, linear-function
// Similar problems: https://atcoder.jp/contests/pakencamp-2020-day2/tasks/pakencamp_2020_day2_e
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        xv: [(i64, i64); n],
        q: usize,
        tlr: [(i64, i64, i64); q],
    }
    const M: i64 = 100_000;
    const B: i64 = 450;
    let mut xs = vec![vec![]; 2 * M as usize + 1];
    for &(x, v) in &xv {
        xs[(v + M) as usize].push(x);
    }
    for v in &mut xs {
        v.sort();
    }
    let mut ans = vec![0; q];
    let mut small = vec![vec![]; B as usize];
    for i in 0..q {
        let (t, l, r) = tlr[i];
        if t < B {
            small[t as usize].push((l, r, i));
            continue;
        }
        let lo = (M - l) / t;
        let hi = r / t;
        for v in -lo..=hi {
            let vec = &xs[(v + M) as usize];
            ans[i] +=
                (vec.upper_bound(&(r - v * t)) - vec.lower_bound(&(l - v * t))) as i64;
        }
    }
    let mut tbl = vec![0; M as usize + 1];
    let mut acc = vec![0; M as usize + 2];
    for t in 0..B {
        for i in 0..M as usize + 1 {
            tbl[i] = 0;
            acc[i + 1] = 0;
        }
        for &(x, v) in &xv {
            let val = x + v * t;
            if val >= 0 && val <= M {
                tbl[val as usize] += 1;
            }
        }
        for i in 0..M as usize + 1 {
            acc[i + 1] = acc[i] + tbl[i];
        }
        for &(l, r, idx) in &small[t as usize] {
            ans[idx] += acc[r as usize + 1] - acc[l as usize];
        }
    }
    for a in ans {
        puts!("{}\n", a);
    }
}
