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
    let a: Vec<i64> = (0 .. n).map(|_| get()).collect();
    let mut goal = a.iter().fold(0, |x, &y| x ^ y);
    const B: usize = 32;
    let mut cnt = [0; B];
    for v in a {
        if v == 0 { continue; }
        let diff = (v ^ (v - 1)).count_ones() as usize - 1;
        cnt[diff] += 1;
    }
    let mut tot = 0;
    for i in (0 .. B).rev() {
        if (goal & 1 << i) == 0 { continue; }
        if cnt[i] == 0 { // impossible
            println!("-1");
            return;
        }
        goal ^= (1 << (i + 1)) - 1;
        tot += 1;
    }
    println!("{}", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
