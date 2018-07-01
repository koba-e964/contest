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
    let n: usize = get();
    let a: Vec<i64> = (0 .. 1 << n).map(|_| get()).collect();
    let mut tap = a;
    let mut ris = vec![0; 1 << n];
    for i in 0 .. n {
        for bits in 0 .. 1 << n {
            if (bits & 1 << i) != 0 {
                let prev = bits ^ 1 << i;
                ris[bits] = max(ris[bits], tap[bits] + tap[prev]);
                tap[bits] = max(tap[bits], tap[prev]);
            }
        }
    }
    let mut ma = 0;
    for i in 1 .. 1 << n {
        ma = max(ma, ris[i]);
        println!("{}", ma);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
