#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Read, Write, BufWriter};
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
    let d = get();
    let g: i64 = get();
    let mut p = vec![0; d];
    let mut c = vec![0; d];
    for i in 0 .. d {
        p[i] = get();
        c[i] = get();
    }
    let mut mi = 1 << 28;
    for bits in 0 .. 1 << d {
        let mut tot = 0;
        let mut cnt = 0;
        for j in 0 .. d {
            if (bits & 1 << j) == 0 { continue; }
            tot += 100 * (j as i64 + 1) * p[j] + c[j];
            cnt += p[j];
        }
        if tot >= g {
            mi = min(mi, cnt);
            continue;
        }
        for i in 0 .. d {
            if (bits & 1 << i) != 0 { continue; }
            let sc = 100 * (i as i64 + 1);
            let need = (g - tot + sc - 1) / sc;
            if need >= p[i] { continue; }
            mi = min(mi, cnt + need);
        }
    }
    println!("{}", mi);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
