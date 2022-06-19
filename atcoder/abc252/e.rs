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
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize, m: usize,
        abc: [(usize1, usize1, i64); m],
    }
    let mut g = vec![vec![]; n];
    for i in 0..m {
        let (a, b, c) = abc[i];
        g[a].push((b, c, i));
        g[b].push((a, c, i));
    }
    const INF: i64 = 1 << 50;
    let mut dist = vec![(INF, 0); n];
    let mut que = BinaryHeap::new();
    que.push((Reverse(0), 0, m));
    while let Some((Reverse(d), v, ei)) = que.pop() {
        if dist[v].0 <= d { continue; }
        dist[v] = (d, ei);
        for &(w, c, ei) in &g[v] {
            que.push((Reverse(d + c), w, ei));
        }
    }
    let mut v = vec![];
    for i in 1..n {
        v.push(dist[i].1 + 1);
    }
    putvec!(v);
}
