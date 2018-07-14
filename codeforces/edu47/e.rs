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


const MOD: i64 = 998244353;

fn solve() {
    let n = get();
    let a: Vec<i64> = (0 .. n).map(|_| get()).collect();
    let mut acc = vec![0; n + 1];
    for i in 0 .. n {
        acc[i + 1] = (acc[i] + a[i]) % MOD;
    }
    let mut ans = 0;
    let mut b = vec![1];
    let mut bsum = 1;
    let mut cur = 1;
    for _ in 1 .. n + 1 {
        let nv = (bsum + cur) % MOD;
        b.push(nv);
        bsum = (bsum + nv) % MOD;
        cur = 2 * cur % MOD;
    }
    for i in 0 .. n {
        ans = (ans + acc[n - i] * b[i]) % MOD;
    }
    println!("{}", ans);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
