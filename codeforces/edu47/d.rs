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

fn fail() -> ! {
    println!("Impossible");
    std::process::exit(0)
}

fn gcd(mut x: usize, mut y: usize) -> usize {
    while y > 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

fn solve() {
    let n: usize = get();
    let m: usize = get();
    if m + 1 < n { fail(); }
    let mut edges = Vec::new();
    for i in 1 .. n + 1 {
        if edges.len() >= m {
            break;
        }
        for j in i + 1 .. n + 1 {
            if edges.len() >= m {
                break;
            }
            if gcd(i, j) == 1 {
                edges.push((i, j));
            }
        }
    }
    if edges.len() < m { fail(); }
    println!("Possible");
    for (a, b) in edges {
        println!("{} {}", a, b);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
