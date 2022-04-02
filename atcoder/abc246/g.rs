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

// Does Takahashi win?
fn dfs(v: usize, par: usize, g: &[Vec<usize>], b: &[bool]) -> i32 {
    let mut num = 0;
    if b[v] {
        num += 1;
    }
    let mut ma = 0;
    for &w in &g[v] {
        if w == par { continue; }
        let sub = dfs(w, v, g, b);
        num += sub;
        ma = max(ma, sub);
    }
    if ma > 0 {
        num -= 1;
    }
    num
}

fn solve() {
    input! {
        n: usize,
        a: [i64; n - 1],
        uv: [(usize1, usize1); n - 1],
    }
    let mut a = a;
    a.insert(0, 0);
    let mut g = vec![vec![]; n];
    for &(u, v) in &uv {
        g[u].push(v);
        g[v].push(u);
    }
    let mut pass = 0;
    let mut fail = 1 << 30;
    while fail - pass > 1 {
        let mid = (fail + pass) / 2;
        let b: Vec<bool> = a.iter().map(|&a| a >= mid).collect();
        let ans = dfs(0, n, &g, &b);
        if ans > 0 {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    println!("{}", pass);
}
