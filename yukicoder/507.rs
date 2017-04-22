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

fn excess(a: &[i64], my: i64, x: usize, m: usize) -> bool {
    let got = my + a[x] + 1;
    let mut halfover = Vec::new();
    let mut under = Vec::new();
    let foo = got / 2;
    let bar = got - got / 2;
    for i in 0 .. a.len() {
        if i == x { continue; }
        if bar <= a[i] {
            halfover.push(a[i] - bar);
        } else {
            under.push(foo - a[i]);
        }
    }
    halfover.sort();
    under.sort();
    if halfover.len() < m {
        return false;
    }
    let mut cnt = 0;
    while let Some(ent) = under.pop() {
        if let Some(&pos) = halfover.last() {
            if pos >= ent {
                halfover.pop().unwrap();
                cnt += 1;
            }
        }
    }
    cnt + halfover.len() / 2 >= m
}

fn solve() {
    let n = get();
    let m: usize = get();
    let my: i64 = get();
    let mut a: Vec<i64> = (1 .. n).map(|_| get()).collect();
    a.sort();
    let mut lo = 0;
    let mut hi = n;
    while hi - lo > 1 {
        let mid = (hi + lo) / 2;
        if excess(&a, my, mid - 1, m) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    println!("{}", if hi == n { -1 } else { a[hi - 1] });
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
