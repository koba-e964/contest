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

/// Generates an Iterator over subsets of univ, in the descending order. 
/// Verified by: http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=3050308
struct SubsetIter { bits: Option<usize>, univ: usize }
impl Iterator for SubsetIter {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        match self.bits {
            None => None,
            Some(bits) => {
                let ans = bits;
                self.bits =
                    if bits == 0 { None }
                else { Some((bits - 1) & self.univ) };
                Some(ans)
            }
        }
    }
}
fn subsets(univ: usize) -> SubsetIter {
    SubsetIter { bits: Some(univ), univ: univ }
}

const INF: i64 = 1 << 50;

fn calc(x: &[(i64, i64)]) -> i64 {
    let n = x.len();
    let mut pt = 0;
    for i in 0..n {
        pt += x[i].1;
    }
    let mut pos = 0;
    let mut part = 0;
    while part * 2 < pt {
        part += x[pos].1;
        pos += 1;
    }
    let mid = x[pos - 1].0;
    let mut tot = 0;
    for i in 0..n {
        tot += (x[i].0 - mid).abs() * x[i].1;
    }
    tot
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize,
        xyp: [(i64, i64, i64); n],
    }
    let mut dp = vec![vec![INF; 1 << n]; n + 1];
    let mut cell = vec![INF; 1 << n];
    let mut none = vec![INF; 1 << n];
    for bits in 1..1 << n {
        let mut x = vec![];
        for i in 0..n {
            if (bits & 1 << i) != 0 {
                x.push((xyp[i].0, xyp[i].2));
            }
        }
        x.sort();
        cell[bits] = min(cell[bits], calc(&x));
        let mut tot = 0;
        for &(x, p) in &x {
            tot += x.abs() * p;
        }
        none[bits] = min(none[bits], tot);
        x.clear();
        for i in 0..n {
            if (bits & 1 << i) != 0 {
                x.push((xyp[i].1, xyp[i].2));
            }
        }
        x.sort();
        cell[bits] = min(cell[bits], calc(&x));
        let mut tot = 0;
        for &(x, p) in &x {
            tot += x.abs() * p;
        }
        none[bits] = min(none[bits], tot);
    }
    dp[0][0] = 0;
    for i in 0..n + 1 {
        for bits in 0..1 << n {
            if bits == 0 {
                continue;
            }
            for sub in subsets(bits) {
                if sub == 0 { continue; }
                if i >= 1 {
                    dp[i][bits] = min(dp[i][bits],
                                      dp[i - 1][bits - sub] + cell[sub]);
                }
                dp[i][bits] = min(dp[i][bits],
                                  dp[i][bits - sub] + none[sub]);
            }
        }
    }
    for i in 0..n + 1 {
        puts!("{}\n", dp[i][(1 << n) - 1]);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
