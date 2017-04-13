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

fn calc(mut n: i64) -> i64 {
    // up to 18 decimal digits
    const B: usize = 18;
    // 1 through (10^18 - 1) / 9
    let mut tbl = [0i64; B];
    tbl[0] = 1;
    for i in 1 .. B {
        tbl[i] = tbl[i - 1] * 10 + 1;
    }
    tbl.reverse();
    let mut ret = 0;
    let mut pick_cnt = 0;
    let mut pos = 0;
    while n > 0 && pick_cnt < 9 && pos < B {
        while pos < B && tbl[pos] > n { pos += 1; }
        if pos == B { break; }
        n -= tbl[pos];
        pick_cnt += 1;
        ret += tbl[pos];
    }
    ret
}

fn solve() {
    let t: usize = get();
    for case_nr in 1 .. t + 1 {
        let n: i64 = get();
        println!("Case #{}: {}", case_nr, calc(n));
    }
    
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
