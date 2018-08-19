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

/// http://hama-du-competitive.hatenablog.com/entry/2017/04/22/185540 wo mimashita
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    let n = get();
    let m: usize = get();
    let q: usize = get();
    let mut pool = Vec::new();
    for i in 0 .. m + q {
        let l: usize = get::<usize>() - 1;
        let r: usize = get::<usize>() - 1;
        pool.push((r, i, l));
    }
    pool.sort();
    let mut ans = vec![0; q];
    let mut seg = vec![0; n];
    for (r, id, l) in pool {
        if id < m {
            seg[l] += 1;
        } else {
            let idx = id - m;
            ans[idx] += seg[l..r+1].iter().sum::<i32>();
        }
    }
    for v in ans { puts!("{}\n", v); }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
