use std::cmp::*;
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
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        t: usize,
        data: [[(i64, i64)]; t],
    }
    for i in 0..t {
        let n = data[i].len();
        let mut f = vec![];
        for &(x, y) in &data[i] {
            f.push(x);
            f.push(y);
        }
        let p = f.split_off(n);
        let p: Vec<usize> = p.into_iter().map(|p| p as usize).collect();
        puts!("Case #{}: {}\n", i + 1, calc(&f, &p));
    }
}

const INF: i64 = 1 << 50;

fn dfs(v: usize, ch: &[Vec<usize>], f: &[i64]) -> [i64; 2] {
    if ch[v].is_empty() {
        return [f[v], 0];
    }
    let mut all = 0;
    let mut ma = -INF;
    for &w in &ch[v] {
        let sub = dfs(w, ch, f);
        all += sub[0];
        ma = max(ma, sub[1] - sub[0]);
    }
    [max(all, all + ma + f[v]), all + ma]
}

fn calc(f: &[i64], p: &[usize]) -> i64 {
    let n = f.len();
    let mut r = vec![];
    let mut ch = vec![vec![]; n];
    for i in 0..n {
        if p[i] == 0 {
            r.push(i);
        } else {
            ch[p[i] - 1].push(i);
        }
    }
    let mut init = vec![];
    for i in 0..n {
        if ch[i].is_empty() { init.push(i); }
    }
    let mut ans = 0;
    for &v in &r {
        let res = dfs(v, &ch, &f);
        ans += res[0];
    }
    ans
}
