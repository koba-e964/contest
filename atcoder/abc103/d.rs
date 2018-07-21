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

const INF: usize = 1 << 24;

fn solve() {
    let n = get();
    let m = get();
    let mut far = vec![INF; n];
    for _ in 0 .. m {
        let a = get::<usize>() - 1;
        let b = get::<usize>() - 1;
        far[a] = min(far[a], b);
    }
    let mut ans = 0;
    let mut pos = n;
    for i in (0 .. n).rev() {
        if far[i] >= INF {
            continue;
        }
        if far[i] > pos {
            continue;
        }
        ans += 1;
        pos = i;
    }
    println!("{}", ans);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
