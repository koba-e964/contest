#[allow(unused_imports)]
use std::collections::*;
use std::io::Read;
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

fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

// Written after the author read the editorial
fn solve() {
    let n: usize = get();
    let mut k: i64 = get();
    // A -> false, B -> true
    let mut s: Vec<bool> = get_word().chars().map(|x| x == 'B').collect();
    assert_eq!(s.len(), n);
    let mut pos = 0;
    for i in 0 .. n {
        s[i] = s[i] ^ ((n + i) % 2 != 1);
    }
    for _ in 0 .. n {
        s.push(false);
    }
    while k > 0 {
        if pos >= n {
            break;
        }
        if s[pos] ^ (n % 2 != 1) { // B
            pos += 1;
        } else { // A
            s[pos] = !s[pos];
        }
        k -= 1;
    }
    if k > 0 && n % 2 == 1 && k % 2 == 1 {
        assert_eq!(pos, n);
        s[n] = !s[n];
    }
    // recons
    let mut out = String::new();
    for i in 0 .. n {
        out.push(if s[pos + i] ^ ((n + i) % 2 != 1) { 'B' } else { 'A' });
    }
    println!("{}", out);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
