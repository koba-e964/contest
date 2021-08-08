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
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    let n: usize = get();
    let m: usize = get();
    let mut stronger = vec![0; n];
    let mut dead = 0;
    for _ in 0..m {
        let a: usize = get::<usize>() - 1;
        let b: usize = get::<usize>() - 1;
        let (a, _b) = if a < b { (a, b) } else { (b, a) };
        stronger[a] += 1;
        if stronger[a] == 1 {
            dead += 1;
        }
    }
    let q: usize = get();
    for _ in 0..q {
        let kind: i32 = get();
        if kind == 1 {
            let a: usize = get::<usize>() - 1;
            let b: usize = get::<usize>() - 1;
            let (a, _b) = if a < b { (a, b) } else { (b, a) };
            stronger[a] += 1;
            if stronger[a] == 1 {
                dead += 1;
            }
        } else if kind == 2 {
            let a: usize = get::<usize>() - 1;
            let b: usize = get::<usize>() - 1;
            let (a, _b) = if a < b { (a, b) } else { (b, a) };
            stronger[a] -= 1;
            if stronger[a] == 0 {
                dead -= 1;
            }
        } else {
            puts!("{}\n", n - dead);
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
