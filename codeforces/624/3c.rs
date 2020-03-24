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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    let t: usize = get();
    for _ in 0..t {
        let n: usize = get();
        let m: usize = get();
        let s: Vec<_> = get_word().chars().collect();
        let p: Vec<usize> = (0..m).map(|_| get::<usize>() - 1).collect();
        let mut ans = vec![0; 26];
        let mut imos = vec![0; n];
        imos[n - 1] = 1;
        for &p in &p {
            imos[p] += 1;
        }
        for i in (0..n - 1).rev() {
            imos[i] += imos[i + 1];
        }
        for i in 0..n {
            let pos = (s[i] as u8 - b'a') as usize;
            ans[pos] += imos[i];
        }
        for i in 0..26 {
            puts!("{}{}", ans[i], if i == 25 { "\n" } else { " " });
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
