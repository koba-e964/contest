#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Read, Write, BufWriter};
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}
fn get_word() -> String {
    let mut stdin = std::io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

const INF: i64 = 1 << 50;

fn dfs(v: usize, s: &[i64], g: &[Vec<usize>], dp: &mut [Vec<i64>]) {
    let mut cur = vec![0, -s[v]];
    for &w in g[v].iter() {
        dfs(w, s, g, dp);
        let mut nxt = vec![-INF; cur.len() + dp[w].len() - 1];
        nxt[0] = 0;
        for i in 1 .. cur.len() {
            for j in 0 .. dp[w].len() {
                nxt[i + j] = max(nxt[i + j], cur[i] + dp[w][j]);
            }
        }
        cur = nxt;
    }
    dp[v] = cur;
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    let n = get();
    let s: Vec<i64> = (0 .. n).map(|_| get()).collect();
    let mut g = vec![Vec::new(); n];
    for _ in 0 .. n - 1 {
        let a = get::<usize>() - 1;
        let b = get::<usize>() - 1;
        g[a].push(b);
    }
    let m = get();
    let mut t: Vec<i64> = (0 .. m).map(|_| get()).collect();
    t.sort();
    t.reverse();
    let mut dp = vec![Vec::new(); n];
    dfs(0, &s, &g, &mut dp);
    let mut t_acc = vec![0; m + 1];
    for i in 0 .. m {
        t_acc[i + 1] = t_acc[i] + t[i];
    }
    let ssum: i64 = s.iter().sum();
    let mut ans = -INF;
    for i in 0 .. min(m + 1, dp[0].len()) {
        ans = max(ans, ssum + dp[0][i] + t_acc[i]);
    }
    puts!("{}\n", ans);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
