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

fn dfs(v: usize, g: &[Vec<usize>],
       st: &mut [usize], en: &mut [usize],
       euler: &mut Vec<usize>) {
    let stidx = euler.len();
    euler.push(v);
    st[v] = stidx;
    for &w in &g[v] {
        dfs(w, g, st, en, euler);
    }
    en[v] = euler.len();
}

fn solve() {
    let n: usize = get();
    let q = get();
    let mut g = vec![Vec::new(); n];
    for i in 1 .. n {
        let u = get::<usize>() - 1;
        g[u].push(i);
    }
    let mut st = vec![0; n];
    let mut en = vec![0; n];
    let mut euler = Vec::new();
    dfs(0, &g, &mut st, &mut en, &mut euler);
    for _ in 0 .. q {
        let u = get::<usize>() - 1;
        let k = get::<usize>() - 1;
        if en[u] - st[u] <= k {
            println!("-1");
        } else {
            println!("{}", euler[st[u] + k] + 1);
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600 * 3 / 2; // 150 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
