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
    let a: Vec<usize> = (0 .. n).map(|_| get::<usize>()).filter(|&x| x < k)
        .collect();
    let n = a.len();
    // DP (O(nk))
    let mut dp = vec![vec![false; k + 1]; n + 1];
    let mut dp_rev = vec![vec![false; k + 1]; n + 1];
    let mut dp_acc = vec![vec![0; k + 1]; n + 1];
    let mut is_necessary = vec![false; n];
    dp[0][0] = true;
    for i in 0 .. n {
        for j in 0 .. k + 1 {
            if dp[i][j] {
                dp[i + 1][j] = true;
                dp[i + 1][min(j + a[i], k)] = true;
            }
        }
    }
    dp_rev[n][0] = true;
    for i in (0 .. n).rev() {
        for j in 0 .. k + 1 {
            if dp_rev[i + 1][j] {
                dp_rev[i][j] = true;
                dp_rev[i][min(j + a[i], k)] = true;
            }
        }
    }
    for i in 0 .. n + 1 {
        // cumulative sum
        let mut sum = 0;
        for j in (0 .. k + 1).rev() {
            sum += if dp_rev[i][j] { 1 } else { 0 };
            dp_acc[i][j] = sum;
        }
    }
    for i in 0 .. n {
        for j in 0 .. k {
            let opp_start = if k > j + a[i] { k - j - a[i] } else { 0 };
            let opp_end = if k - 1 > j { k - j - 1 } else { 0 };
            if dp[i][j] {
                let cnt = dp_acc[i + 1][opp_start] - dp_acc[i + 1][opp_end + 1];
                if cnt >= 1 {
                    is_necessary[i] = true;
                }
            }
        }
    }
    println!("{}", is_necessary.iter().filter(|&x| !x).count());
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
