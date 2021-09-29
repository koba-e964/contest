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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn dfs(g: &[Vec<(usize, i64)>], v: usize, dp: &mut [i64]) {
    if dp[v] >= 0 {
        return;
    }
    let mut me = 0;
    for &(w, c) in &g[v] {
        dfs(g, w, dp);
        me = max(me, dp[w] + c);
    }
    dp[v] = me;
}

fn solve() {
    input! {
        n: usize, m: usize,
        abc: [(usize, usize, i64); m],
    }
    let mut g = vec![vec![]; n];
    let mut revg = vec![vec![]; n];
    for &(a, b, c) in &abc {
        g[a].push((b, c));
        revg[b].push((a, c));
    }
    let mut dp1 = vec![-1; n];
    let mut dp2 = vec![-1; n];
    dfs(&revg, n - 1, &mut dp1);
    dfs(&g, 0, &mut dp2);
    let t = dp1[n - 1];
    let p = (0..n).filter(|&i| dp1[i] + dp2[i] < t).count();
    println!("{} {}/{}", t, p, n);
}
