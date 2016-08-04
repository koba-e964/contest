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

fn main() {
    let h: i32 = get();
    let w: i32 = get();
    let s: Vec<Vec<char>> = (0 .. h).map(|_| getline().chars().collect()).collect();
    let mut dp: Vec<Vec<i32>> = (0 .. h).map(|_| vec![100_000; w as usize]).collect();
    for i in 0 .. h {
        for j in 0 .. w {
            let iu = i as usize;
            let ju = j as usize;
            if s[iu][ju] == '.' {
                dp[iu][ju] = 0;
            } else {
                let mut m = 100_000;
                m = min(m, if iu > 0 {
                    dp[iu - 1][ju]
                } else { 0 });
                m = min(m, if ju > 0 {
                    dp[iu][ju - 1]
                } else { 0 });
                m = min(m, if iu > 0 && ju > 0 {
                    dp[iu - 1][ju - 1]
                } else { 0 });
                dp[iu][ju] = m + 1;
            }
        }
    }
    let mut ma: i32 = 0;
    for i in 0 .. h {
        ma = max(ma, *dp[i as usize].iter().max().unwrap());
    }
    println!("{}", (ma + 1) / 2);
}
