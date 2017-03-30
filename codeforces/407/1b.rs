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

fn dfs(v: usize, edges: &[Vec<usize>], vis: &mut [bool]) {
    vis[v] = true;
    for &w in edges[v].iter() {
        if vis[w] { continue; }
        dfs(w, edges, vis);
    }
}

fn solve() {
    let n = get();
    let m = get();
    let mut edges = vec![Vec::new(); n];
    let mut arb = None;
    let mut numloop: i64 = 0;
    let mut loopy = vec![false; n];
    for _ in 0 .. m {
        let u = get::<usize>() - 1;
        let v = get::<usize>() - 1;
        edges[u].push(v);
        if u != v {
            edges[v].push(u);
        } else {
            numloop += 1;
            loopy[u] = true;
        }
        if arb.is_none() {
            arb = Some(u);
        }
    }
    // Are all the PATHS connected?
    let arb = arb.unwrap();
    let mut vis = vec![false; n];
    dfs(arb, &edges, &mut vis);
    for i in 0 .. n {
        if !vis[i] && edges[i].len() > 0 {
            // not connected
            println!("0");
            return;
        }
    }
    // connected.
    let mut tot: i64 = 0;
    // Two non-loops
    for i in 0 .. n {
        let deg = edges[i].len() as i64 - if loopy[i] { 1 } else { 0 };
        tot += deg * (deg - 1) / 2;
    }
    tot += numloop * (m - numloop); // loop + non-loop
    tot += numloop * (numloop - 1) / 2;
    println!("{}", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
