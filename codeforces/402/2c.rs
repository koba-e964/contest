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
    let k = get();
    let a: Vec<i64> = (0 .. n).map(|_| get()).collect();
    let b: Vec<i64> = (0 .. n).map(|_| get()).collect();
    let mut srt: Vec<(i64, usize)> = Vec::new();
    for i in 0 .. n {
        srt.push((a[i] - b[i], i));
    }
    srt.sort();
    let mut tot: i64 = 0;
    for i in 0 .. k {
        let v = srt[i].1;
        tot += a[v];
    }
    for i in k .. n {
        let v = srt[i].1;
        tot += min(a[v], b[v]);
    }
    println!("{}", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
