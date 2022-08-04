use std::cmp::*;
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
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

const INF: i64 = 1 << 60;

fn dfs(v: usize, par: usize, g: &[Vec<(usize, i64)>], d: &[usize]) -> [i64; 2] {
    let mut diff = vec![];
    let mut sum = 0;
    for &(w, c) in &g[v] {
        if w == par { continue; }
        let sub = dfs(w, v, g, d);
        sum += sub[0];
        diff.push(sub[1] - sub[0] + max(c, 0));
    }
    diff.sort(); diff.reverse();
    if d[v] == 0 {
        return [sum, -INF];
    }
    for i in 0..d[v] - 1 {
        sum += max(diff[i], 0);
    }
    if diff.len() == d[v] - 1 {
        [sum, sum]
    } else {
        [sum + max(diff[d[v] - 1], 0), sum]
    }
}

fn solve() {
    input! {
        n: usize,
        d: [usize; n],
        uvw: [(usize1, usize1, i64); n - 1],
    }
    let mut g = vec![vec![]; n];
    for &(u, v, w) in &uvw {
        g[u].push((v, w));
        g[v].push((u, w));
    }
    let res = dfs(0, n, &g, &d);
    println!("{}", res[0]);
}
