#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

const INF: i64 = 1 << 45;

// dp[i][j]: 頂点 i において、 自分と非連結な物を j 個作るために必要な操作の回数
fn dfs(ch: &[Vec<(usize, i64)>], v: usize) -> Vec<i64> {
    let mut dp = vec![0];
    for &(w, h) in &ch[v] {
        let sub = dfs(ch, w);
        let mut ep = vec![INF; dp.len() + sub.len()];
        for i in 0..dp.len() {
            for j in 0..sub.len() {
                if sub[j] > h { continue; }
                let val = dp[i] + h;
                ep[i + j + 1] = min(ep[i + j + 1], val);
                let val = dp[i] + sub[j];
                ep[i + j] = min(ep[i + j], val);
            }
        }
        dp = ep;
    }
    dp
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        ph: [(usize, i64); n - 1],
    }
    let mut ch = vec![vec![]; n];
    for i in 0..n - 1 {
        let (p, h) = ph[i];
        ch[p].push((i + 1, h));
    }
    let res = dfs(&ch, 0);
    let mut ma = 0;
    for i in 0..n {
        if res[i] < INF { ma = i; }
    }
    puts!("{}\n", ma + 1);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
