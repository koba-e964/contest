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

fn dfs1(v: usize, par: usize, g: &[Vec<usize>], uni: &[bool], dp1: &mut [usize]) {
    let mut ans = if uni[v] { 1 } else { 0 };
    for &w in &g[v] {
        if w == par { continue; }
        dfs1(w, v, g, uni, dp1);
        ans += dp1[w];
    }
    dp1[v] = ans;
}

fn dfs2(v: usize, par: usize, k: usize, g: &[Vec<usize>], dp1: &[usize]) -> i64 {
    let mut ans = 0;
    for &w in &g[v] {
        if w == par { continue; }
        ans += dfs2(w, v, k, g, dp1);
        ans += min(dp1[w], 2 * k - dp1[w]) as i64;
    }
    ans
}

fn solve() {
    input! {
        n: usize, k: usize,
        u: [usize1; 2 * k],
        xy: [(usize1, usize1); n - 1],
    }
    let mut g = vec![vec![]; n];
    for &(x, y) in &xy {
        g[x].push(y);
        g[y].push(x);
    }
    let mut uni = vec![false; n];
    for u in u {
        uni[u] = true;
    }
    let mut dp1 = vec![0; n];
    dfs1(0, n, &g, &uni, &mut dp1);
    let ans = dfs2(0, n, k, &g, &dp1);
    println!("{}", ans);
}
