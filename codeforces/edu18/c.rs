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

fn output(vec: Vec<i32>) {
    for v in vec {
        print!("{}", v);
    }
    println!("");
}

fn calc(n: &[i32], rem: i32, cnt: i32) -> Option<Vec<i32>> {
    let mut remcnt = 0;
    let w = n.len();
    let mut picked = vec![true; w];
    for i in (0 .. w).rev() {
        if remcnt >= cnt { continue; }
        if rem == n[i] % 3 {
            remcnt += 1;
            picked[i] = false;
        }
    }
    if remcnt < cnt {
        return None;
    }
    let ret: Vec<i32> = (0 .. w).filter(|&i| picked[i])
        .map(|i| n[i]).collect();
    if ret.is_empty() {
        return None;
    }
    let w = ret.len();
    let mut pos = 0;
    while pos < w {
        if ret[pos] == 0 {
            pos += 1;
        } else {
            break;
        }
    }
    if pos == w {
        pos -= 1;
    }
    Some(ret[pos .. w].to_vec())
}

fn solve() {
    let n: Vec<i32> = get_word().bytes().map(|x| (x - b'0') as i32)
        .collect();
    let mut rem = 0;
    for &d in n.iter() {
        rem = (rem + d) % 3;
    }
    if rem == 0 {
        output(n);
        return;
    }
    let mut ma = (0, vec![-1]);
    if let Some(x) = calc(&n, rem, 1) {
        ma = max(ma, (x.len(), x));
    }
    if let Some(x) = calc(&n, 3 - rem, 2) {
        ma = max(ma, (x.len(), x));
    }
    output(ma.1);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
