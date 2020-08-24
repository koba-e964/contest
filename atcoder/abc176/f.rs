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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize,
        a: [usize1; 3 * n],
    }
    const INF: i64 = 1 << 40;
    let mut dp = vec![vec![-INF; n]; n];
    let mut rmax = vec![-INF; n];
    let mut cmax = vec![-INF; n];
    let mut gl_ma = -INF;
    if n == 1 {
        puts!("{}\n", if a[0] == a[1] && a[1] == a[2] { 1 } else { 0 });
        return;
    }
    for i in 0..5 {
        for j in 0..i {
            let mut rest = vec![];
            for k in 0..5 {
                if k != i && k != j {
                    rest.push(a[k]);
                }
            }
            let v = if rest[0] == rest[1] && rest[1] == rest[2] {
                1
            } else {
                0
            };
            dp[a[i]][a[j]] = v;
            rmax[a[i]] = max(rmax[a[i]], v);
            cmax[a[j]] = max(cmax[a[j]], v);
            gl_ma = max(gl_ma, v);
        }
    }
    let mut global: i64 = 0;
    let mut upd = vec![];
    for i in 1..n - 1 {
        // [3 * i + 2, 3 * i + 5)
        let v = if a[3 * i + 2] == a[3 * i + 3]
            && a[3 * i + 3] == a[3 * i + 4] {
                1
            } else {
                0
            };
        global += v;
        for j in 0..3 {
            for k in 0..j {
                let rest = a[3 * i + 2 + 3 - j - k];
                let aj = a[3 * i + 2 + j];
                let ak = a[3 * i + 2 + k];
                upd.push((aj, ak, max(gl_ma - v, dp[rest][rest] + 1 - v)));
            }
        }
        for j in 0..3 {
            let aj = a[3 * i + 2 + j];
            let ok = a[3 * i + 2 + ((j + 1) % 3)] == a[3 * i + 2 + ((j + 2) % 3)];
            let ak = a[3 * i + 2 + ((j + 1) % 3)];
            for r in 0..n {
                upd.push((r, aj, rmax[r] - v));
                if ok {
                    upd.push((r, aj, dp[r][ak] - v + 1));
                }
            }
            for c in 0..n {
                upd.push((aj, c, cmax[c] - v));
                if ok {
                    upd.push((aj, c, dp[ak][c] - v + 1));
                }
            }
        }
        for (x, y, val) in upd.drain(..) {
            dp[x][y] = max(dp[x][y], val);
            rmax[x] = max(rmax[x], val);
            cmax[y] = max(cmax[y], val);
            gl_ma = max(gl_ma, val);
        }
    }/*
    eprintln!("global = {}", global);
    for i in 0..n {
        eprintln!("dp[{}] = {:?}", i, dp[i]);
    }*/
    let mut ma = 0;
    for i in 0..n {
        for j in 0..n {
            ma = max(ma, dp[i][j]);
        }
    }
    let rest = a[3 * n - 1];
    ma = max(ma, dp[rest][rest] + 1);
    puts!("{}\n", ma + global);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
