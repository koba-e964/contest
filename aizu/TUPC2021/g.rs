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
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn dfs(g: &[Vec<(usize, u8)>], v: usize, par: usize, dp: &mut [u32], x: u32) {
    dp[v] = x;
    for &(w, c) in &g[v] {
        if w == par { continue; }
        dfs(g, w, v, dp, x ^ 1 << c);
    }
}

fn solve() {
    input! {
        n: usize,
        uvc: [(usize1, usize1, char); n - 1],
    }
    let mut dp = vec![0; n];
    let mut g = vec![vec![]; n];
    for &(u, v, c) in &uvc {
        let c = c as u8 - b'a';
        g[u].push((v, c));
        g[v].push((u, c));
    }
    dfs(&g, 0, n, &mut dp, 0);
    let mut hm = HashMap::new();
    let mut tot = 0i64;
    for i in 0..n {
        tot += *hm.get(&dp[i]).unwrap_or(&0);
        for j in 0..26 {
            tot += *hm.get(&(dp[i] ^ 1 << j)).unwrap_or(&0);
        }
        *hm.entry(dp[i]).or_insert(0) += 1;
    }
    println!("{}", tot);
}
