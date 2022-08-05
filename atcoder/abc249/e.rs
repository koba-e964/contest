use std::cmp::*;
use std::io::Read;

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
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

fn main() {
    let n: usize = get();
    let p: i64 = get();
    let add = |a: &mut i64, b: i64| {
        *a += b;
        if *a >= p { *a -= p; }
    };
    let mut dp = vec![vec![0; n]; n + 1];
    let mut acc = vec![vec![0; n]; n + 2];
    dp[0][0] = 1;
    acc[1][0] = 1;
    for i in 1..n + 1 {
        for j in 0..n {
            if j >= 2 {
                let mut tmp = acc[i][j - 2] - acc[max(i, 10) - 9][j - 2];
                add(&mut tmp, p);
                add(&mut dp[i][j], tmp);
            }
            if j >= 3 {
                let mut tmp = acc[max(i, 10) - 9][j - 3] - acc[max(i, 100) - 99][j - 3];
                add(&mut tmp, p);
                add(&mut dp[i][j], tmp);
            }
            if j >= 4 {
                let mut tmp = acc[max(i, 100) - 99][j - 4] - acc[max(i, 1000) - 999][j - 4];
                add(&mut tmp, p);
                add(&mut dp[i][j], tmp);
            }
            if j >= 5 {
                let mut tmp = acc[max(i, 1000) - 999][j - 5] - acc[max(i, 10000) - 9999][j - 5];
                add(&mut tmp, p);
                add(&mut dp[i][j], tmp);
            }
            dp[i][j] = dp[i][j] * 25 % p;
            if i < 10 && j == 2 {
                add(&mut dp[i][j], 26);
            }
            if i >= 10 && i < 100 && j == 3 {
                add(&mut dp[i][j], 26);
            }
            if i >= 100 && i < 1000 && j == 4 {
                add(&mut dp[i][j], 26);
            }
            if i >= 1000 && j == 5 {
                add(&mut dp[i][j], 26);
            }
            acc[i + 1][j] = acc[i][j];
            add(&mut acc[i + 1][j], dp[i][j]);
        }
    }
    let mut ans = 0;
    for i in 0..n {
        add(&mut ans, dp[n][i]);
    }
    println!("{}", ans);
}
