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
        n: usize, m: usize, q: usize, l: i64,
        abc: [(usize1, usize1, i8); m],
        t: [usize1; q],
    }
    let mut g = vec![vec![]; n];
    for (a, b, c) in abc {
        g[a].push((b, c));
        g[b].push((a, c));
    }
    let mut dist = vec![vec![l; n]; 30];
    for i in 0..30 {
        dist[i] = vec![l + 1 - (1 << i); n];
    }
    let mut que = BinaryHeap::new();
    que.push((Reverse(0), 0, 0));
    while let Some((Reverse(d), lv, v)) = que.pop() {
        if dist[lv][v] <= d { continue; }
        dist[lv][v] = d;
        for &(w, c) in &g[v] {
            if c == 1 {
                que.push((Reverse(d + (1 << lv)), lv, w));
            } else if lv < 29 {
                que.push((Reverse(d), lv + 1, w));
            }
        }
    }
    for t in t {
        let mut mi = l + 1;
        for i in 0..30 {
            mi = min(mi, dist[i][t] + (1 << i));
        }
        if mi > l {
            puts!("Large\n");
        } else {
            puts!("{}\n", mi);
        }
    }
}
