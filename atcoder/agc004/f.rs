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

fn color(v: usize, par: usize, g: &[Vec<usize>], col: &mut [usize], c: usize) {
    col[v] = c;
    for &w in &g[v] {
        if par == w { continue; }
        color(w, v, g, col, 1 - c);
    }
}

fn dfs(v: usize, par: usize, g: &[Vec<usize>], col: &[usize], flow: &mut [i64]) -> (i64, i64) {
    let (mut b, mut w) = (0, 0);
    if col[v] == 0 {
        w += 1;
    } else {
        b += 1;
    }
    for &ww in &g[v] {
        if par == ww { continue; }
        let (sb, sw) = dfs(ww, v, g, col, flow);
        b += sb;
        w += sw;
    }
    flow[v] = b - w;
    (b, w)
}

// Partial solution implemented after reading the editorial.
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        m: usize,
        ab: [(usize1, usize1); m],
    }
    assert_eq!(m, n - 1);
    let mut g = vec![vec![]; n];
    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }
    let mut col = vec![0; n];
    color(0, n, &g, &mut col, 0);
    let blackc = col.iter().filter(|&&x| x == 0).count();
    if 2 * blackc != n {
        puts!("-1");
        return;
    }
    let mut flow = vec![0; n];
    dfs(0, n, &g, &col, &mut flow);
    let mut ans = 0;
    for i in 1..n {
        ans += flow[i].abs();
    }
    puts!("{}\n", ans);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
