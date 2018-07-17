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
    let x: Vec<i32> = (0..n).map(|_| get()).collect();
    let mut y = Vec::new();
    for i in 0 .. x.len() {
        y.push((x[i], i));
    }
    y.sort_unstable();
    y.reverse();
    let mut cand = Vec::new();
    let mut tot = 0;
    for i in 0 .. k {
        cand.push(y[i].1);
        tot += y[i].0;
    }
    cand.sort_unstable();
    println!("{}", tot);
    let mut ans = vec![0; k];
    for i in 0 .. k {
        ans[i] = if i + 1 == k { n } else { cand[i + 1] } - cand[i];
    }
    ans[0] += cand[0];
    for i in 0 .. cand.len() {
        print!("{}{}", ans[i], if i + 1 == cand.len() { "\n" } else { " " });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
