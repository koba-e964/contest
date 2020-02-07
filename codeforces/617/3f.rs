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

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

fn dfs(g: &[Vec<(usize, usize)>], v: usize, par: usize, to: usize,
       val: i64,
       dp: &mut [i64])
       -> Result<(), ()> {
    if v == to {
        return Err(());
    }
    for &(w, idx) in &g[v] {
        if w == par {
            continue;
        }
        if dfs(g, w, v, to, val, dp).is_err() {
            dp[idx] = max(dp[idx], val);
            return Err(());
        }
    }
    Ok(())
}

fn dfs2(g: &[Vec<(usize, usize)>], v: usize, par: usize, to: usize,
       dp: &[i64])
       -> Result<(), i64> {
    if v == to {
        return Err(1 << 60);
    }
    for &(w, idx) in &g[v] {
        if w == par {
            continue;
        }
        if let Err(val) = dfs2(g, w, v, to, dp) {
            let val = min(dp[idx], val);
            return Err(val);
        }
    }
    Ok(())
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        xy: [(usize1, usize1); n - 1],
        m: usize,
        abg: [(usize1, usize1, i64); m],
    }
    let mut g = vec![vec![]; n];
    for i in 0..n - 1 {
        let (x, y) = xy[i];
        g[x].push((y, i));
        g[y].push((x, i));
    }
    let mut dp = vec![1; n - 1];
    for &(a, b, c) in &abg {
        dfs(&g, a, n, b, c, &mut dp).err().unwrap();
    }
    for &(a, b, c) in &abg {
        if Err(c) != dfs2(&g, a, n, b, &dp) {
            puts!("-1\n");
            return;
        }
    }
    for i in 0..n - 1 {
        puts!("{}{}", dp[i], if i + 1 == n - 1 { "\n" } else { " " });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
