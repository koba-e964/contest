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
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn dfs1(v: usize, par: usize, g: &[Vec<(usize, i32)>], dp1: &mut [[usize; 2]]) {
    let mut ma = [0; 2];
    for &(w, dir) in &g[v] {
        if par == w { continue; }
        dfs1(w, v, g, dp1);
        for b in 0..2 {
            if (dir & 1 << b) != 0 {
                ma[b] = max(ma[b], dp1[w][b] + 1);
            }
        }
    }
    dp1[v] = ma;
}

fn dfs2(v: usize, par: usize, g: &[Vec<(usize, i32)>], dp1: &[[usize; 2]],
        dp2: &mut [usize], inher: [usize; 2]) {
    let mut ma = [vec![], vec![]];
    for &(w, dir) in &g[v] {
        let mut k;
        if par == w {
            k = inher;
        } else {
            k = dp1[w];
            for b in 0..2 {
                if (dir & 1 << b) != 0 {
                    k[b] += 1;
                } else {
                    k[b] = 0;
                }
            }
        }
        for b in 0..2 {
            ma[b].push((k[b], w));
        }
    }
    for b in 0..2 {
        ma[b].sort();
        ma[b].reverse();
    }
    dp2[v] = max(ma[0][0].0, ma[1][0].0);
    if ma[0][0].1 != ma[1][0].1 {
        dp2[v] = ma[0][0].0 + ma[1][0].0;
    } if ma[0].len() >= 2 {
        dp2[v] = max(dp2[v], ma[0][1].0 + ma[1][0].0);
        dp2[v] = max(dp2[v], ma[1][1].0 + ma[0][0].0);
    }
    for &(w, dir) in &g[v] {
        if par == w { continue; }
        let mut subinher = [0; 2];
        for b in 0..2 {
            if ma[b][0].1 != w {
                subinher[b] = ma[b][0].0;
            } else if ma[b].len() >= 2 {
                subinher[b] = ma[b][1].0;
            }
            if (dir & 1 << (1 - b)) != 0 {
                subinher[b] += 1;
            } else {
                subinher[b] = 0;
            }
        }
        dfs2(w, v, g, dp1, dp2, subinher);
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        abt: [(usize1, usize1, i32); n - 1],
    }
    let mut g = vec![vec![]; n];
    for &(a, b, t) in &abt {
        g[a].push((b, if t == 1 { 1 } else { 3 }));
        g[b].push((a, if t == 1 { 2 } else { 3 }));
    }
    let mut dp1 = vec![[0; 2]; n];
    let mut dp2 = vec![0; n];
    dfs1(0, n, &g, &mut dp1);
    dfs2(0, n, &g, &dp1, &mut dp2, [0; 2]);
    let ma: usize = dp2.into_iter().max().unwrap();
    puts!("{}\n", ma - 1);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
