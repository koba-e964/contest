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

const W: usize = 140;
const BL: usize = (20010 + W - 1) / W;

#[derive(Clone, Debug)]
enum B {
    E(i64),
    D(i64, Vec<i64>),
}
use B::*;

fn calc(v: Vec<(i64, i64)>) -> Vec<i64> {
    let mut dp = vec![E(0); BL];
    let n = v.len();
    let mut ans = vec![0; n];
    let mut oldsum = 0;
    for i in 0..n {
        let (a, b) = v[i];
        let a = a as usize;
        // update(a, b)
        let mut sum = oldsum;
        let abl = a / W;
        for j in 0..abl {
            match dp[j] {
                E(x) => {
                    if b > x {
                        sum += (b - x) * W as i64;
                        dp[j] = E(b);
                    }
                }
                D(mut locsum, ref v) => {
                    if v[0] < b {
                        sum += b * W as i64 - locsum;
                        dp[j] = E(b);
                    } else if v[W - 1] < b {
                        let mut v = v.clone();
                        for j in (0..W).rev() {
                            if v[j] >= b {
                                break;
                            }
                            sum += b - v[j];
                            locsum += b - v[j];
                            v[j] = b;
                        }
                        dp[j] = D(locsum, v);
                    }
                }
            }
        }
        if abl * W < a {
            let (mut locsum, mut v) =
                match &dp[abl] {
                    &E(x) =>
                        (x * W as i64, vec![x; W]),
                    &D(l, ref v) => (l, v.clone()),
                };
            let oldlocsum = locsum;
            for j in (0..a - abl * W).rev() {
                if v[j] >= b {
                    break;
                }
                locsum += b - v[j];
                v[j] = b;
            }
            dp[abl] = D(locsum, v);
            sum += locsum - oldlocsum;
        }
        ans[i] += sum - oldsum;
        oldsum = sum;
    }
    ans
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize,
        xy: [[(i64, i64); 2]; n],
    }
    let mut ans = vec![0i64; n];
    let mut xy = xy;
    for i in 0..n {
        xy[i][0].0 *= -1;
        xy[i][0].1 *= -1;
    }
    for i in 0..2 {
        for j in 0..2 {
            let mut v = vec![(0, 0); n];
            for k in 0..n {
                v[k] = (xy[k][i].0, xy[k][j].1);
            }
            let sub = calc(v);
            for k in 0..n {
                ans[k] += sub[k];
            }
        }
    }
    for i in 0..n {
        puts!("{}\n", ans[i]);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
