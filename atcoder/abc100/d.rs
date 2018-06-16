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
    let n = get();
    let m = get();
    let mut x: Vec<i64> = vec![0; n];
    let mut y: Vec<i64> = vec![0; n];
    let mut z: Vec<i64> = vec![0; n];
    for i in 0 .. n {
        x[i] = get();
        y[i] = get();
        z[i] = get();
    }
    let mut ma = 0;
    for a in [-1, 1].iter() {
        for b in [-1, 1].iter() {
            for c in [-1, 1].iter() {
                let mut iroha = vec![0; n];
                for i in 0 .. n {
                    iroha[i] = a * x[i] + b * y[i] + c * z[i];
                }
                iroha.sort();
                iroha.reverse();
                let tot = iroha[0 .. m].iter().sum();
                ma = max(ma, tot);
            }
        }
    }
    println!("{}", ma);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
