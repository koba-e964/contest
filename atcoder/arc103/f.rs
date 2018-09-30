#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
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

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn dfs(v: usize, par: Option<usize>, d: i64, g: &[Vec<usize>]) -> (i64, i64) {
    let mut tot = 0;
    let mut ssz = 1;
    for &w in g[v].iter() {
        if Some(w) == par { continue; }
        let (sub, sz) = dfs(w, Some(v), d + 1, g);
        tot += sub + sz;
        ssz += sz;
    }
    (tot, ssz)
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    input!{
        n: usize,
        d: [i64; n],
    }
    let mut pool = vec![];
    for i in 0 .. n {
        pool.push((d[i], i));
    }
    pool.sort();
    pool.reverse();
    let mut edges = vec![];
    let mut gad: HashMap<i64, Vec<(usize, i64)>> = HashMap::new();
    for (p, idx) in pool {
        if let Some(vs) = gad.remove(&p) {
            let mut sz = 0;
            for &(v, s) in vs.iter() {
                sz += s;
                edges.push((v, idx));
            }
            sz += 1;
            let targ = p + 2 * sz - n as i64;
            if sz != n as i64 {
                gad.entry(targ).or_insert(vec![]).push((idx, sz));
            }
        } else {
            let sz = 1;
            let targ = p + 2 * sz - n as i64;
            gad.entry(targ).or_insert(vec![]).push((idx, sz));
        }
    }
    let mut g = vec![Vec::new(); n];
    for &(u, v) in &edges {
        g[u].push(v);
        g[v].push(u);
    }
    let (d0, _) = dfs(0, None, 0, &g);
    if gad.len() == 0 && d[0] == d0 {
        for (u, v) in edges {
            puts!("{} {}\n", u + 1, v + 1);
        }
    } else {
        puts!("-1\n");
        return;
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
