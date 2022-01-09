use std::cmp::*;
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes.by_ref().map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize, q: usize, k: usize,
        a: [usize1; k],
        uv: [(usize1, usize1); m],
        st: [(usize1, usize1); q],
    }
    let mut g = vec![vec![]; n];
    for &(u, v) in &uv {
        g[u].push(v);
        g[v].push(u);
    }
    let mut dist = vec![];
    for i in 0..k {
        let mut que = VecDeque::new();
        que.push_back((0, a[i]));
        let mut tmp = vec![1 << 30; n];
        while let Some((d, v)) = que.pop_front() {
            if tmp[v] <= d { continue; }
            tmp[v] = d;
            for &w in &g[v] {
                que.push_back((d + 1, w));
            }
        }
        dist.push(tmp);
    }
    for (s, t) in st {
        let mut mi = 1 << 30;
        for i in 0..k {
            mi = min(mi, dist[i][s] + dist[i][t]);
        }
        puts!("{}\n", mi);
    }
}
