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

fn calc(x: Vec<(i32, i32)>, kind: i32) -> i32 {
    let mut cur = 0;
    let mut count = 0;
    let mut iroha = Vec::new();
    for (k, v) in x {
        if k == kind {
            cur = v;
            while let Some(x) = iroha.pop() {
                if x >= cur {
                    iroha.push(x);
                    break;
                }
                count += 1;
            }
            continue;
        }
        if v < cur {
            count += 1;
        } else {
            iroha.push(v);
        }
    }
    count
}
fn calc_overtake(x: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut iroha = Vec::new();
    for k in x {
        if k == 2 {
            while let Some(v) = iroha.pop() {
                if v == 6 {
                    count += 1;
                } else {
                    iroha.push(v);
                    break;
                }
            }
            continue;
        }
        iroha.push(k);
    }
    count
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    let n = get();
    let mut speed = Vec::new();
    let mut overtake = Vec::new();
    for _ in 0 .. n {
        let k: i32 = get();
        if k % 2 == 1 {
            if k == 1 || k == 3 {
                let s = get::<i32>();
                speed.push((k, s));
            } else {
                speed.push((5, 1000));
            }
        } else {
            overtake.push(k);
        }
    }
    puts!("{}\n", calc(speed, 1) + calc_overtake(overtake));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
