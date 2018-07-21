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

fn solve() {
    loop {
        let n = get();
        let m: usize = get();
        if n == 0 && m == 0 { break; }
        let r = get();
        let mut hist = vec![Vec::new(); n];
        for _ in 0 .. r {
            let t: usize = get();
            let n: usize = get::<usize>() - 1;
            let m: usize = get::<usize>() - 1;
            get_word();
            hist[n].push((m, t));
        }
        const T: usize = 1500;
        let mut imos = vec![vec![0i32; T]; m];
        for hist in &hist {
            for j in 0 .. hist.len() / 2 {
                let m = hist[2 * j].0;
                imos[m][hist[2 * j].1] += 1;
                imos[m][hist[2 * j + 1].1] -= 1;
            }
        }
        for i in 0 .. m {
            for j in 0 .. T - 1 {
                imos[i][j + 1] += imos[i][j];
            }
            for j in 0 .. T {
                imos[i][j] = min(imos[i][j], 1);
            }
            for j in 0 .. T - 1 {
                imos[i][j + 1] += imos[i][j];
            }
        }
        let q = get();
        for _ in 0 .. q {
            let ts: usize = get();
            let te: usize = get();
            let m: usize = get();
            println!("{}", imos[m - 1][te - 1] - imos[m - 1][ts - 1]);
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
