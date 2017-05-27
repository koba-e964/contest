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

fn calc(a: i64, b: i64) -> i64 {
    if b == 0 {
        return 1;
    }
    let mut highest = 0;
    let mut snd = 100;
    for k in (0 .. 63).rev() {
        if (b & 1 << k) != 0 {
            highest = k;
            break;
        }
    }
    let brem = b ^ 1i64 << highest;
    if brem == 0 {
        return if a != 0 {
            2 * (b - a) + 1
        } else {
            2 * b
        };
    }
    for k in (0 .. 63).rev() {
        if (brem & 1 << k) != 0 {
            snd = k;
            break;
        }
    }
    if a < 2 << snd {
        return (2 << highest) - a;
    }
    let hh = 1i64 << highest;
    (hh - a) + (2i64 << snd) + (hh - a)
}
fn solve() {
    let mut a: i64 = get();
    let mut b: i64 = get();
    for k in (0 .. 63).rev() {
        if (a & 1 << k) == (b & 1 << k) {
            a &= !(1 << k);
            b &= !(1 << k);
        } else {
            break;
        }
    }
    // essential part
    println!("{}", calc(a, b));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
