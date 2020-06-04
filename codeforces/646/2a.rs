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

fn solve() {
    let t: usize = get();
    for _ in 0..t {
        let n: usize = get();
        let x: usize = get();
        let a: Vec<usize> = (0..n).map(|_| {
            let b = get::<usize>();
            b % 2
        }).collect();
        let one: usize = a.iter().sum::<usize>();
        let zero = n - one;
        let mut ok = false;
        for i in 0..x + 1 {
            if i > zero || x - i > one {
                continue;
            }
            if (x - i) % 2 == 1 {
                ok = true;
            }
        }
        println!("{}", if ok { "Yes" } else { "No" });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
