#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Read;
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

const INF: i64 = 1 << 57;

fn solve() {
    let a = get();
    let n = get();
    let m = get();
    let mut rainy = vec![false; a];
    for _ in 0 .. n {
        let l: usize = get();
        let r = get();
        for i in l .. r {
            rainy[i] = true;
        }
    }
    let mut x = vec![0usize; m];
    let mut p = vec![0i64; m];
    let mut kirika = vec![(INF, m); a + 1];
    for i in 0 .. m {
        x[i] = get();
        p[i] = get();
        let r = &mut kirika[x[i]];
        *r = min(*r, (p[i], i));
    }
    let mut dp = vec![vec![INF; m + 1]; a + 1];
    dp[0][0] = 0;
    for i in 0 .. a {
        let mut mi = INF;
        for j in 0 .. m + 1 {
            mi = min(mi, dp[i][j]);
        }
        if kirika[i].0 < INF {
            dp[i][kirika[i].1 + 1] = mi;
        }
        dp[i][0] = mi;
        for j in 0 .. m + 1 {
            if !rainy[i] || j != 0 {
                dp[i + 1][j] = min(dp[i + 1][j], dp[i][j] + (if j != 0 { p[j - 1] } else { 0 }));
            }
        }
    }
    let mut mi = INF;
    for i in 0 .. m + 1 {
        mi = min(mi, dp[a][i]);
    }
    println!("{}", if mi >= INF { -1 } else { mi });
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
