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

fn check(v: usize, c: bool, edges: &[Vec<usize>], col: &mut [bool],
         vis: &mut [bool], conn: &mut Vec<usize>) -> bool {
    if vis[v] {
        return c == col[v];
    }
    vis[v] = true;
    col[v] = c;
    conn.push(v);
    let mut bip = true;
    for &w in edges[v].iter() {
        bip &= check(w, !c, edges, col, vis, conn);
    }
    bip
}


fn bipartite(edges: &[Vec<usize>]) -> (usize, usize, usize) {
    let n = edges.len();
    let mut col = vec![false; n];
    let mut vis = vec![false; n];
    let mut bip = 0;
    let mut odd = 0;
    let mut single = 0;
    for i in 0 .. n {
        if vis[i] { continue; }
        let mut conn = Vec::new();
        let is_b = check(i, false, edges, &mut col, &mut vis, &mut conn);
        if !is_b || conn.len() == 1 {
            if conn.len() == 1 {
                single += 1;
            } else {
                odd += 1;
            }
        } else {
            bip += 1;
        }
    }
    (bip, odd, single)
}

fn solve() {
    let n = get();
    let m = get();
    let mut edges = vec![Vec::new(); n];
    for _ in 0 .. m {
        let u = get::<usize>() - 1;
        let v = get::<usize>() - 1;
        edges[u].push(v);
        edges[v].push(u);
    }
    let (bip, odd, single) = bipartite(&edges);
    let bip = bip as i64;
    let odd = odd as i64;
    let single = single as i64;
    let n = n as i64;
    println!("{}", 2 * bip * (bip + odd) + odd * odd
             + n * n - (n - single) * (n - single));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
