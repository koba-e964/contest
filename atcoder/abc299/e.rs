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
        n: usize, m: usize,
        uv: [(usize1, usize1); m],
        k: usize,
        pd: [(usize1, i32); k],
    }
    let mut g = vec![vec![]; n];
    for (u, v) in uv {
        g[u].push(v);
        g[v].push(u);
    }
    let mut white = vec![false; n];
    for &(p, dt) in &pd {
        if dt == 0 { continue; }
        let mut vis = vec![false; n];
        let mut que = VecDeque::new();
        que.push_back((0, p));
        while let Some((d, v)) = que.pop_front() {
            if vis[v] { continue; }
            vis[v] = true;
            if d < dt - 1 {
                for &w in &g[v] {
                    que.push_back((d + 1, w));
                }
            }
        }
        for i in 0..n {
            white[i] |= vis[i];
        }
    }
    const INF: i32 = 1 << 28;
    let mut dist = vec![INF; n];
    let mut que = VecDeque::new();
    for i in 0..n {
        if !white[i] {
            que.push_back((0, i));
        }
    }
    while let Some((d, v)) = que.pop_front() {
        if dist[v] <= d { continue; }
        dist[v] = d;
        for &w in &g[v] {
            que.push_back((d + 1, w));
        }
    }
    for &(p, d) in &pd {
        if dist[p] != d {
            puts!("No\n");
            return;
        }
    }
    puts!("Yes\n");
    let mut s = "".to_string();
    for i in 0..n {
        s.push(if white[i] { '0' } else { '1' });
    }
    puts!("{}\n", s);
}
