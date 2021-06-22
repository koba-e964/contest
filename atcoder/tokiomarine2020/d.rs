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

fn knapsack_linear_sort(a: &[(i64, usize)]) -> Vec<(usize, i64)> {
    let n = a.len();
    let mut ans = vec![(0, 0); 1 << n];
    for i in 0..n {
        let (v, c) = a[i];
        let mut pos0 = 1 << i;
        let mut pos1 = 1 << i;
        for j in (0..2 << i).rev() {
            if pos0 == 0 || pos1 == 0 {
                break;
            }
            if ans[pos0 - 1].0 < ans[pos1 - 1].0 + c {
                ans[j] = (ans[pos1 - 1].0 + c, ans[pos1 - 1].1 + v);
                pos1 -= 1;
            } else {
                ans[j] = ans[pos0 - 1];
                pos0 -= 1;
            }
        }
        for j in 0..pos1 {
            ans[j].0 += c;
            ans[j].1 += v;
        }
    }
    for i in 1..1 << n {
        ans[i].1 = max(ans[i].1, ans[i - 1].1);
    }
    ans
}

// Tags: sqrt-decomposition, half-enumeration, linear-sorting
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        vw: [(i64, usize); n],
        vl: [(usize1, usize)],
    }
    for &(mut v, l) in &vl {
        let mut bags = vec![];
        loop {
            bags.push(vw[v]);
            if v == 0 {
                break;
            }
            v = (v - 1) / 2;
        }
        let m = bags.len();
        let fst = knapsack_linear_sort(&bags[..m / 2]);
        let snd = knapsack_linear_sort(&bags[m / 2..]);
        let mut pos = 0;
        let mut ma = 0;
        for i in (0..fst.len()).rev() {
            while pos < snd.len() && snd[pos].0 + fst[i].0 <= l {
                pos += 1;
            }
            if pos > 0 {
                ma.chmax(fst[i].1 + snd[pos - 1].1);
            }
        }
        puts!("{}\n", ma);
    }
}
