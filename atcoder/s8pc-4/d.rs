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

fn dfs1(v: usize, par: Option<usize>, edges: &[Vec<usize>],
        dp: &mut [Vec<f64>]) -> f64 {
    let mut tot = 0.0;
    let mut trail = 0.0;
    for i in 0 .. edges[v].len() {
        let w = edges[v][i];
        if Some(w) == par { continue; }
        let res = dfs1(w, Some(v), edges, dp);
        tot += res;
        dp[v][i] = res + 1.0;
        trail += 1.0;
    }
    if trail != 0.0 {
        tot / trail + 1.0
    } else {
        0.0
    }
}
fn dfs2(v: usize, par: Option<usize>, edges: &[Vec<usize>],
        dp: &mut [Vec<f64>], promise: f64) {
    let mut tot = 0.0;
    let mut trail = 0.0;
    for i in 0 .. edges[v].len() {
        let w = edges[v][i];
        if Some(w) == par {
            dp[v][i] = promise;
            tot += promise;
        } else {
            tot += dp[v][i];
        }
        trail += 1.0;
    }
    for i in 0 .. edges[v].len() {
        let w = edges[v][i];
        if Some(w) == par { continue; }
        let tmp = tot - dp[v][i];
        dfs2(w, Some(v), edges, dp,
             (if trail != 1.0 { tmp / (trail - 1.0) } else { 0.0 }) + 1.0);
    }
}


fn solve() {
    let n = get();
    let mut edges = vec![Vec::new(); n];
    for _ in 0 .. n - 1 {
        let u = get::<usize>() - 1;
        let v = get::<usize>() - 1;
        edges[u].push(v);
        edges[v].push(u);
    }
    let mut dp = vec![Vec::new(); n];
    for i in 0 .. n {
        dp[i] = vec![-1.0 / 0.0; edges[i].len()];
    }
    dfs1(0, None, &edges, &mut dp);
    dfs2(0, None, &edges, &mut dp, 0.0 / 0.0);
    //println!("dp = {:?}", dp);
    for i in 0 .. n {
        let mut tot = 0.0;
        for &val in dp[i].iter() {
            tot += val;
        }
        println!("{}", tot / edges[i].len() as f64);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
