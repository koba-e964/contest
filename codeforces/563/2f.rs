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
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
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

fn ask_d(v: usize) -> i32 {
    println!("d {}", v + 1);
    let x: i32 = get();
    if x == -1 {
        std::process::exit(1);
    }
    x
}

fn ask_s(v: usize) -> usize {
    println!("s {}", v + 1);
    let x: i32 = get();
    if x == -1 {
        std::process::exit(1);
    }
    (x - 1) as usize
}

fn size(v: usize, par: usize, g: &[Vec<usize>], sz: &mut [usize],
        dep: &mut [i32], pa: &mut [usize]) {
    let mut s = 1;
    pa[v] = par;
    for &w in &g[v] {
        if w == par { continue; }
        dep[w] = dep[v] + 1;
        size(w, v, g, sz, dep, pa);
        s += sz[w];
    }
    sz[v] = s;
}

fn get_bary(v: usize, par: &[usize], g: &[Vec<usize>], sz: &[usize],
            tot: usize) -> usize {
    let mut nxt = None;
    let mut ch = vec![];
    for &w in &g[v] {
        if w == par[v] { continue; }
        ch.push(w);
        if 2 * sz[w] >= tot {
            nxt = Some(w);
            break;
        }
    }
    if let Some(nxt) = nxt {
        get_bary(nxt, par, g, sz, tot)
    } else {
        if ch.is_empty() { v } else { ch[0] }
    }
}

fn dfs(v: usize, par: &[usize], g: &[Vec<usize>], sz: &[usize], dep: &[i32],
       dist: i32) -> usize {
    if dist == 0 {
        return v;
    }
    if dist == 1 {
        return ask_s(v);
    }
    let bary = get_bary(v, par, g, sz, sz[v]);
    let dbary = if v == bary { dist } else { ask_d(bary) };
    let x = (dist - dbary + dep[bary] - dep[v]) / 2;
    let mut cur = bary;
    for _ in (dep[v] + x..dep[bary]).rev() {
        cur = par[cur];
    }
    let rest = dist - x;
    if rest == 0 {
        return cur;
    }
    let next = ask_s(cur);
    dfs(next, par, g, sz, dep, rest - 1)
}

fn solve() {
    let n: usize = get();
    let mut g = vec![vec![]; n];
    for _ in 0..n - 1 {
        let u = get::<usize>() - 1;
        let v = get::<usize>() - 1;
        g[u].push(v);
        g[v].push(u);
    }
    let mut sz = vec![0; n];
    let mut dep = vec![0; n];
    let mut par = vec![0; n];
    size(0, n, &g, &mut sz, &mut dep, &mut par);
    let dist0 = ask_d(0);
    let v = dfs(0, &par, &g, &sz, &dep, dist0);
    println!("! {}", v + 1);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
