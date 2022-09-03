use std::cmp::*;
use std::collections::*;
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
    input! {
        n: usize, m: usize,
        a: [i64; n],
        uv: [(usize1, usize1); m],
    }
    let mut tot = vec![0; n];
    let mut alive = vec![true; n];
    let mut g = vec![vec![]; n];
    for &(u, v) in &uv {
        g[u].push(v);
        g[v].push(u);
        tot[u] += a[v];
        tot[v] += a[u];
    }
    let mut que = BinaryHeap::new();
    for i in 0..n {
        que.push((Reverse(tot[i]), i));
    }
    let mut ma = 0;
    while let Some((Reverse(c), idx)) = que.pop() {
        if !alive[idx] { continue; }
        alive[idx] = false;
        ma = max(ma, c);
        for &w in &g[idx] {
            tot[w] -= a[idx];
            que.push((Reverse(tot[w]), w));
        }
    }
    println!("{}", ma);
}
