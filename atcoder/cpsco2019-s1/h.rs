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

fn induce(a: &[i64], b: &[i64], x: i64) -> i64 {
    let n = a.len();
    let m = b.len();
    let mut ma = -1;
    let mut ma_c = 0;
    let mut tmp = vec![(0, 0); n];
    for i in (0..n).rev() {
        if ma < a[i] {
            ma = a[i];
            ma_c = 1;
        } else if ma == a[i] {
            ma_c += 1;
        }
        tmp[i] = (ma, ma_c);
    }
    let mut pos = 0;
    let mut hm = HashMap::new();
    let mut tot = 0i64;
    for i in (0..n).rev() {
        let (ma, ma_c) = tmp[i];
        while pos < m && b[pos] <= ma {
            *hm.entry(b[pos]).or_insert(0) += 1;
            pos += 1;
        }
        let val = hm.get(&(x - ma - a[i])).cloned().unwrap_or(0);
        tot += ma_c * val;
    }
    tot
}

fn rec(a: &[i64], x: i64) -> i64 {
    let n = a.len();
    if n <= 1 {
        return if 3 * a[0] == x { 1 } else { 0 };
    }
    let mid = n / 2;
    let mut ans = 0;
    ans += rec(&a[..mid], x);
    ans += rec(&a[mid..], x);
    ans += induce(&a[..mid], &a[mid..], x);
    let mut l = a[..mid].to_vec();
    let mut r = a[mid..].to_vec();
    l.reverse();
    r.reverse();
    ans += induce(&r, &l, x);
    ans
}

// Tags: divide-and-conquer
// The author read the answer before implementing this.
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, x: i64,
        a: [i64; n],
    }
    let tot = rec(&a, x);
    puts!("{}\n", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
