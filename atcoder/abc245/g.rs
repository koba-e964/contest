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
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize, m: usize, k: u32, l: usize,
        a: [u32; n],
        b: [usize1; l],
        uvc: [(usize1, usize1, i64); m],
    }
    let mut g = vec![vec![]; n];
    for (u, v, c) in uvc {
        g[u].push((v, c));
        g[v].push((u, c));
    }
    let mut que = BinaryHeap::new();
    const INF: i64 = 1 << 50;
    let mut dist = vec![[(INF, k + 1); 2]; n];
    for b in b {
        que.push((Reverse(0), a[b], b));
    }
    while let Some((Reverse(d), col, v)) = que.pop() {
        if dist[v][1].0 <= d { continue; }
        if dist[v][0].1 == col || dist[v][1].1 == col {
            if dist[v][0].1 == col {
                if dist[v][0].0 <= d { continue; }
                dist[v][0].0 = d;
            }
            if dist[v][1].1 == col {
                if dist[v][1].0 <= d { continue; }
                dist[v][1].0 = d;
            }
        } else {
            dist[v][1] = (d, col);
        }
        if dist[v][0].0 > dist[v][1].0 {
            dist[v].swap(0, 1);
        }
        for &(w, c) in &g[v] {
            que.push((Reverse(d + c), col, w));
        }
    }
    let mut ans = vec![0; n];
    for i in 0..n {
        let tmp = if dist[i][0].1 == a[i] { dist[i][1].0 } else { dist[i][0].0 };
        ans[i] = if tmp >= INF { -1 } else { tmp };
    }
    putvec!(ans);
}
