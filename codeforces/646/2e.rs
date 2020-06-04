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

const INF: i64 = 1 << 55;

fn dfs1(v: usize, par: usize, g: &[Vec<usize>], abc: &[(i64, i32, i32)],
        a: &mut [i64], b: &mut [i32], c: &mut [i32], acc: i64) {
    let (x, y, z) = abc[v];
    let acc = min(acc, x);
    a[v] = acc;
    b[v] = y;
    c[v] = z;
    for &w in &g[v] {
        if w == par { continue; }
        dfs1(w, v, g, abc, a, b, c, acc);
    }
}

fn dfs2(v: usize, par: usize, g: &[Vec<usize>],
        a: &[i64], b: &[i32], c: &[i32]) -> (i64, i64, i64) {
    let mut z01 = 0;
    let mut z10 = 0;
    let mut sum = 0;
    if b[v] == 0 && c[v] == 1 {
        z01 += 1;
    }
    if b[v] == 1 && c[v] == 0 {
        z10 += 1;
    }
    for &w in &g[v] {
        if w == par { continue; }
        let (x, y, z) = dfs2(w, v, g, a, b, c);
        sum += x;
        z01 += y;
        z10 += z;
    }
    let r = min(z01, z10);
    sum += a[v] * r;
    z01 -= r;
    z10 -= r;
    (sum, z01, z10)
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize,
        abc: [(i64, i32, i32); n],
        uv: [(usize1, usize1); n - 1],
    }
    let mut g = vec![vec![]; n];
    for &(u, v) in &uv {
        g[u].push(v);
        g[v].push(u);
    }
    let mut a = vec![INF; n];
    let mut b = vec![0; n];
    let mut c = vec![0; n];
    dfs1(0, n, &g, &abc, &mut a, &mut b, &mut c, INF);
    let (x, y, z) = dfs2(0, n, &g, &a, &b, &c);
    if y != 0 || z != 0 {
        puts!("-1\n");
    } else {
        puts!("{}\n", x * 2);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
