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
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn dfs(v: usize, g: &[Vec<(usize, i64)>], vis: &mut [bool], dp: &mut [i64], x: i64) -> Result<(), ()> {
    if vis[v] {
        return if x == dp[v] {
            Ok(())
        } else {
            Err(())
        };
    }
    vis[v] = true;
    dp[v] = x;
    for &(w, c) in &g[v] {
        dfs(w, g, vis, dp, x ^ c)?;
    }
    Ok(())
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize,
        aby: [(usize1, usize1, i64); m],
    }
    let mut g = vec![vec![]; n];
    for &(a, b, y) in &aby {
        g[a].push((b, y));
        g[b].push((a, y));
    }
    let mut vis = vec![false; n];
    let mut dp = vec![0; n];
    for i in 0..n {
        if !vis[i] {
            if dfs(i, &g, &mut vis, &mut dp, 0).is_err() {
                puts!("-1\n");
                return;
            }
        }
    }
    for i in 0..n {
        puts!("{}\n", dp[i]);
    }
}
