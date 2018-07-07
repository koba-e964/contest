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

fn dfs(v: usize, a: i64, b: i64, g: &[Vec<(usize, i64)>],
       vis: &mut [bool], coef: &mut [(i64, i64)],
       cand: &mut Vec<i64>) -> bool {
    if vis[v] {
        if coef[v].0 == a {
            return coef[v].1 == b;
        }
        if (coef[v].1 - b) % (coef[v].0 - a) != 0 {
            return false;
        }
        cand.push(-(coef[v].1 - b) / (coef[v].0 - a));
        return true;
    }
    vis[v] = true;
    coef[v] = (a, b);
    for &(w, s) in g[v].iter() {
        if !dfs(w, -a, s - b, g, vis, coef, cand) {
            return false;
        }
    }
    return true;
}

const INF: i64 = 1 << 61;

fn solve() {
    let n = get();
    let m = get();
    let mut g = vec![Vec::new(); n];
    for _ in 0 .. m {
        let u = get::<usize>() - 1;
        let v = get::<usize>() - 1;
        let s: i64 = get();
        g[u].push((v, s));
        g[v].push((u, s));
    }
    let mut vis = vec![false; n];
    let mut coef = vec![(0, 0); n];
    let mut cand = Vec::new();
    if !dfs(0, 1, 0, &g, &mut vis, &mut coef, &mut cand) {
        println!("0");
        return;
    }
    cand.sort();
    cand.dedup();
    //println!("cand = {:?}", cand);
    if cand.len() >= 2 {
        println!("0");
        return;
    }
    let mut lo = 1; let mut hi = INF;
    for i in 0 .. n {
        if coef[i].0 == 1 {
            lo = max(lo, -coef[i].1 + 1);
        } else {
            hi = min(hi, coef[i].1 - 1);
        }
    }
    if cand.len() == 1 {
        let x = cand[0];
        println!("{}", if lo <= x && x <= hi { 1 } else { 0 });
        return;
    }
    println!("{}", if lo <= hi { hi - lo + 1 } else { 0 });
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
