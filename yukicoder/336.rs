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
    let mut dp = vec![0; n + 1];
    let mut dp2 = vec![0; n + 1]; // half
    if n <= 2 {
        println!("0");
        return;
    }
    let mut comb = vec![Vec::new(); n + 1];
    comb[0] = vec![1; 1];
    for i in 1 .. n + 1 {
        let mut row = vec![0; i + 1];
        row[0] = 1;
        row[i] = 1;
        for j in 1 .. i {
            row[j] = (comb[i - 1][j] + comb[i - 1][j - 1]) % MOD;
        }
        comb[i] = row;
    }
    dp[0] = 0;
    dp2[0] = 0;
    dp[1] = 1;
    dp2[1] = 1;
    for i in 2 .. n + 1 {
        let mut sum = 0;
        sum += 2 * dp2[i - 1];
        sum %= MOD;
        for j in 1 .. i - 1 {
            let tmp = dp2[j] * dp2[i - j - 1] % MOD;
            let tmp = tmp * comb[i - 1][j] % MOD;
            sum += tmp;
            sum %= MOD;
        }
        dp[i] = sum;
        dp2[i] = sum * (MOD + 1) / 2 % MOD;
    }
    println!("{}", dp[n]);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
