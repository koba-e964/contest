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
    let a: Vec<_> = get_word().chars().collect();
    let b: Vec<_> = get_word().chars().collect();
    let mut ans = 0;
    for i in 0 .. (n + 1) / 2 {
        let mut hm = HashMap::new();
        *hm.entry(a[i]).or_insert(0) += 1;
        *hm.entry(b[i]).or_insert(0) -= 1;
        if 2 * i + 1 != n {
            *hm.entry(a[n - 1 - i]).or_insert(0) += 1;
            *hm.entry(b[n - 1 - i]).or_insert(0) -= 1;
        }
        let mut p1 = 0;
        let mut m1 = 0;
        for (_, v) in hm {
            if v == 1 { p1 += 1; }
            if v == -1 { m1 += 1; }
        }
        if p1 == 2 && m1 == 0 {
            ans += 1;
        } else {
            ans += max(p1, m1);
        }
    }
    println!("{}", ans);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
