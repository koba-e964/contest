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

fn solve() {
    let n: usize = get();
    let ni = n as i64;
    let mut dp = vec![0; n + 1];
    let mut acc = vec![0; n + 1];
    dp[0] = 1;
    dp[1] = ni;
    acc[0] = 1;
    acc[1] = 1 + ni;
    for i in 2 .. n + 1 {
        let mut tot = (ni - 1) * (ni - 1) % MOD;
        tot += dp[i - 1];
        if i >= 3 {
            tot = (tot + acc[i - 3]) % MOD;
        }
        tot += (n + 1 - i) as i64;
        dp[i] = tot % MOD;
        acc[i] = (acc[i - 1] + tot) % MOD;
    }
    println!("{}", dp[n]);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
