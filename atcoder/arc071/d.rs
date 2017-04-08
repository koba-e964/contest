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

const MOD: i64 = 1_000_000_007;

fn calc(a: &[i64]) -> i64 {
    let n = a.len();
    let mut tot = 0;
    let mut acc = 0;
    for i in 0 .. n {
        tot += (a[i] + MOD) * i as i64 - acc + MOD;
        tot %= MOD;
        acc = (acc + a[i] + MOD) % MOD;
    }
    tot
}

fn solve() {
    let n = get();
    let m = get();
    let x: Vec<i64> = (0 .. n).map(|_| get()).collect();
    let y: Vec<i64> = (0 .. m).map(|_| get()).collect();
    println!("{}", calc(&x) * calc(&y) % MOD);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
