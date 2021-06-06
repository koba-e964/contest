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

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn opt(d: i64) -> i64 {
    if d <= 1 {
        return d;
    }
    let mut lo = 1;
    let mut hi = d;
    while hi - lo > 1 {
        let mid = (hi + lo) / 2;
        if mid * mid <= d {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let mut ma = (d, 0);
    for x in max(0, lo - 4)..lo + 4 {
        ma.chmin((x + d / (x + 1), x));
    }
    ma.1
}

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
        n: usize, m: usize,
        abcd: [(usize1, usize1, i64, i64); m],
    }
    let mut g = vec![vec![]; n];
    for &(a, b, c, d) in &abcd {
        g[a].push((b, c, d));
        g[b].push((a, c, d));
    }
    let mut que = BinaryHeap::new();
    const INF: i64 = 1 << 50;
    let mut dist = vec![INF; n];
    que.push((Reverse(0), 0));
    while let Some((Reverse(ct), v)) = que.pop() {
        if dist[v] <= ct {
            continue;
        }
        dist[v] = ct;
        for &(w, c, d) in &g[v] {
            let x = opt(d);
            let x = max(x, ct);
            let tmp = x + c + d / (x + 1);
            que.push((Reverse(tmp), w));
        }
    }
    puts!("{}\n", if dist[n - 1] >= INF { -1 } else { dist[n - 1] });
}
