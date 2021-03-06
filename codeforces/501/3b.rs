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
    let mut s: Vec<_> = get_word().chars().collect();
    let t: Vec<_> = get_word().chars().collect();
    let mut freq = [0; 26];
    for &c in &s {
        freq[c as usize - 'a' as usize] += 1;
    }
    for &c in &t {
        freq[c as usize - 'a' as usize] -= 1;
    }
    for &c in &freq {
        if c != 0 {
            println!("-1");
            return;
        }
    }
    let mut ops = Vec::new();
    for i in 0 .. n {
        if s[i] == t[i] { continue; }
        let mut idx = n;
        for j in i + 1 .. n {
            if t[i] == s[j] {
                idx = j;
                break;
            }
        }
        for k in (i .. idx).rev() {
            ops.push(k);
            s.swap(k, k + 1);
        }
    }
    println!("{}", ops.len());
    for (i, c) in ops.iter().enumerate() {
        print!("{}{}", c + 1, if i + 1 == ops.len() { "\n" } else { " " });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
