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

fn sort_powset(a: &[i64]) -> Vec<Vec<i64>> {
    let n = a.len();
    let mut ans = vec![(0, 0); 1 << n];
    for i in 0..n {
        let mut pos0 = 0;
        let mut pos1 = 0;
        let offset_in = (1 << n) - (1 << i);
        let mut out = (1 << n) - (1 << (i + 1));
        while pos0 < 1 << i && pos1 < 1 << i {
            if ans[offset_in + pos0].0 < ans[offset_in + pos1].0 + a[i] {
                ans[out] = ans[offset_in + pos0];
                pos0 += 1;
            } else {
                ans[out] = (ans[offset_in + pos1].0 + a[i], ans[offset_in + pos1].1 + 1);
                pos1 += 1;
            }
            out += 1;
        }
        while pos0 < 1 << i {
            ans[out] = ans[offset_in + pos0];
            pos0 += 1;
            out += 1;
        }
        while pos1 < 1 << i {
            ans[out] = (ans[offset_in + pos1].0 + a[i], ans[offset_in + pos1].1 + 1);
            pos1 += 1;
            out += 1;
        }
    }
    let mut ret = vec![vec![]; n + 1];
    for &(a, b) in &ans {
        ret[b].push(a);
    }
    ret
}

// Tags: sqrt-decomposition, linear-time-sorting, merge-sort
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize, k: usize, p: i64,
        a: [i64; n],
    }
    let fst = sort_powset(&a[..n / 2]);
    let snd = sort_powset(&a[n / 2..]);
    let mut tot = 0;
    for i in 0..min(n / 2, k) + 1 {
        let j = k - i;
        if j > n - n / 2 {
            continue;
        }
        let mut pos = 0;
        for &f in fst[i].iter().rev() {
            while pos < snd[j].len() && f + snd[j][pos] <= p {
                pos += 1;
            }
            tot += pos;
        }
    }
    puts!("{}\n", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
