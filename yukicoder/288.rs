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
            if res.is_err() || res.ok().unwrap() == 0 || u8b[0] <= ' ' as u8 {
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


fn solve(a: &[i64], v: i64) -> Option<i64> {
    if v < 0 {
        return None;
    }
    let n = a.len();
    const W: usize = 250000;
    const INF: i32 = 0x3fff_ffff;
    let mut dp: Vec<Vec<i32>> = vec![vec![INF; W]; 2];
    dp[0][0] = 0;
    for i in 0 .. n {
        let t = i % 2;
        for j in 0 .. W {
            dp[1 - t][j] = INF;
        }
        let cur = a[i] as usize;
        for j in 0 .. W {
            let mut mi = dp[t][j];
            if j >= cur {
                mi = min(mi, dp[1 - t][j - cur] + 1);
            }
            dp[1 - t][j] = mi;
        }
    }
    if v < W as i64 {
        let tmp = dp[n % 2][v as usize];
        if tmp >= INF {
            return None;
        }
        return Some(tmp as i64);
    }
    let biggest = a[n - 1] as i64;
    let diff = (v + 1 - W as i64 + biggest - 1) / biggest * biggest;
    assert!(v - diff < W as i64);
    let tmp = dp[n % 2][(v - diff) as usize];
    if tmp >= INF {
        return None;
    }
    return Some(tmp as i64 + diff / biggest);
}


fn main() {
    let n: usize = get();
    let m: i64 = get();
    let a: Vec<i64> = (0 .. n).map(|_| get()).collect();
    let k: Vec<i64> = (0 .. n).map(|_| get()).collect();
    let mut diff: i64 = 0;
    for i in 0 .. n {
        diff += a[i] * k[i];
    }
    diff -= m;
    let result = solve(&a, diff);
    println!("{}", result.unwrap_or(-1));
}
