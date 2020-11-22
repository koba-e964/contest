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

fn get_or(a: usize, b: usize) -> usize {
    println!("OR {} {}", a + 1, b + 1);
    let or: usize = get();
    or
}

fn solve() {
    let n: usize = get();
    let mut d = vec![0; n];
    let mut occ = vec![vec![]; n];
    occ[0].push(0);
    for i in 1..n {
        println!("XOR 1 {}", i + 1);
        d[i] = get();
        occ[d[i]].push(i);
    }
    let mut fst = 0;
    let mut found = false;
    for i in 0..n {
        if occ[i].len() >= 2 {
            let a = occ[i][0];
            let b = occ[i][1];
            let or = get_or(a, b);
            fst = i ^ or;
            found = true;
            break;
        }
    }
    if !found {
        let or1 = get_or(occ[0][0], occ[1][0]) & (n - 2);
        let or2 = get_or(occ[1][0], occ[n - 1][0]) & 1;
        fst = or1 | (or2 ^ 1);
    }
    print!("!");
    for i in 0..n {
        print!(" {}", fst ^ d[i]);
    }
    println!();
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
