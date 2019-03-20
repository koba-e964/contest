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

fn dfs(v: usize, par: usize, g: &[Vec<(usize, usize)>], col: &mut [usize],
       r: usize, c: usize) {
    let ignore = g[v].len() > r;
    let mut nxt = 0;
    if c == 0 { nxt = 1; }
    for &(w, idx) in &g[v] {
        if par == w { continue; }
        let inher = if ignore { 0 } else { nxt };
        col[idx % g.len()] = inher; 
        dfs(w, v, g, col, r, inher);
        nxt += 1;
        if nxt == c { nxt += 1; }
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
        k: usize,
        ab: [(usize1, usize1); n - 1],
    }
    let mut deg = vec![0; n];
    let mut g = vec![vec![]; n];
    for (i, &(a, b)) in ab.iter().enumerate() {
        g[a].push((b, i));
        g[b].push((a, n + i));
        deg[a] += 1;
        deg[b] += 1;
    }
    deg.sort();
    let r = deg[n - k - 1];
    drop(deg);
    let mut col = vec![0; n - 1];
    dfs(0, n, &g, &mut col, r, r);
    puts!("{}\n", r);
    for i in 0..n - 1 {
        puts!("{}{}", col[i] + 1, if i + 1 == n - 1 { "\n" } else { " " });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
