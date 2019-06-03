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

// I read the editorial.
const INF: usize = 1 << 28;

fn dfs(v: usize, par: usize, g: &[Vec<usize>], has: &[bool]) -> (usize, usize, usize) {
    let my = if has[v] { 1 } else { 0 };
    let mut sum = my;
    let mut high = 0;
    let mut rest = vec![];
    for &w in &g[v] {
        if par == w { continue; }
        let (sub, l, h) = dfs(w, v, g, has);
        high += h + sub;
        sum += sub;
        rest.push((l + sub, h + sub));
    }
    if !rest.is_empty() {
        let mut mi = INF;
        let mut sol = false;
        let mut lh = (0, 0);
        for i in 0..rest.len() {
            let (l, h) = rest[i];
            if l > high - h {
                sol = true;
                lh = (l, h);
                break;
            }
        }
        if sol {
            let (l, h) = lh;
            mi = min(mi, l - (high - h));
        } else {
            mi = min(mi, high % 2);
        }
        return (sum, mi, high);
    }
    (sum, 0, 0)
}


fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        s: chars,
        ab: [(usize1, usize1); n - 1],
    }
    let s: Vec<bool> = s.into_iter().map(|c| c == '1').collect();
    let mut g = vec![vec![]; n];
    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }
    let mut mi = INF;
    for i in 0..n {
        let ret = dfs(i, n, &g, &s);
        let (_, low, high) = ret;
        if low == 0 {
            mi = min(mi, high / 2);
        }
    }
    if mi >= INF {
        puts!("-1\n");
    } else {
        puts!("{}\n", mi);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
