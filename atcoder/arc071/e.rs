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

fn tbl(s: &[u8]) -> Vec<i32> {
    let n = s.len();
    let mut ret = vec![0; n + 1];
    for i in 0 .. n {
        let tmp = (s[i] - b'A') as i32 + 1;
        ret[i + 1] = (ret[i] + tmp) % 3;
    }
    ret
}

fn solve() {
    let s: Vec<u8> = get_word().bytes().collect();
    let t: Vec<u8> = get_word().bytes().collect();
    // A = 1, B = 2, mod 3
    let mut acc1 = tbl(&s);
    let mut acc2 = tbl(&t);
    let q = get();
    for _ in 0 .. q {
        let a = get::<usize>() - 1;
        let b: usize = get();
        let c = get::<usize>() - 1;
        let d: usize = get();
        let sum1 = (acc1[b] + 2 * acc1[a]) % 3;
        let sum2 = (acc2[d] + 2 * acc2[c]) % 3;
        println!("{}", if sum1 == sum2 { "YES" } else { "NO" });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
