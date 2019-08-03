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
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    let n: usize = get();
    let a: Vec<i64> = (0..n).map(|_| get()).collect();
    let q: usize = get();
    let mut last = vec![(0, 0); n];
    for i in 0..n {
        last[i].1 = a[i];
    }
    let mut two: Vec<(usize, i64)> = vec![];
    for i in 1..q + 1 {
        let ty: i32 = get();
        if ty == 1 {
            let p: usize = get::<usize>() - 1;
            let x: i64 = get();
            last[p] = (i, x);
        } else {
            let x: i64 = get();
            while let Some(ev) = two.pop() {
                if ev.1 > x {
                    two.push(ev);
                    break;
                }
            }
            two.push((i, x));
        }
    }
    let mut ans = vec![0; n];
    for i in 0..n {
        let idx = two.binary_search(&last[i]).err().unwrap();
        if idx < two.len() && two[idx].1 >= last[i].1 {
            ans[i] = two[idx].1;
        } else {
            ans[i] = last[i].1;
        }
        puts!("{}\n", ans[i]);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
