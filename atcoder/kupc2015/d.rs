#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::*;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
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
            let ret = std::string::String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}
fn parse<T: std::str::FromStr>(s: &str) -> T { s.parse::<T>().ok().unwrap() }

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { parse(&get_word()) }

fn main() {
    let n = get();
    let a: Vec<i64> = (0 .. n).map(|_| get()).collect();
    let b: Vec<i64> = (0 .. n).map(|_| get()).collect();
    let mut bmax = b;
    for i in 1 .. n {
        bmax[i] = max(bmax[i], bmax[i - 1]);
    }
    let mut dp = vec![None; n + 1]; // (#steps to get there, money Takahashi has)
    dp[0] = Some((0usize, 0i64));
    for i in 1 .. n + 1 {
        if let Some((ss, ee)) = dp[i - 1] {
            let mut expected = ee + a[i - 1];
            let mut steps = ss + 1;
            if expected < 0 {
                if bmax[i - 1] <= 0 {
                    dp[i] = None;
                    continue;
                }
                let s = (-expected + bmax[i - 1] - 1) / bmax[i - 1];
                if steps as i64 + s > n as i64 {
                    dp[i] = None; // invalid
                    continue;
                }
                expected += s * bmax[i - 1];
                steps += s as usize
            }
            dp[i] = Some((steps, expected));
        }
    }

    let mut ma: i64 = 0;
    for i in 0 .. n + 1 {
        match dp[i] {
            Some((step, money)) if step <= n => {
                ma = max(ma, money +
                         if i < n { bmax[i] * (n - step) as i64} else { 0 });
            },
            _ => {},
        }
    }
    println!("{}", ma);
}
