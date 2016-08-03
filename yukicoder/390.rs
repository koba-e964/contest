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


const N: usize = 1_000_001;
fn main() {
    let n: usize = get();
    let x: Vec<usize> = (0 .. n).map(|_| get()).collect();
    let mut tbl: Vec<bool> = (0 .. N).map(|_| false).collect();
    for i in x {
        tbl[i] = true;
    }
    let mut dp: Vec<i32> = (0 .. N).map(|_| 0).collect();
    for i in (1 .. N).rev() {
        if tbl[i] {
            dp[i] = 1;
            for j in 2 .. N / i {
                dp[i] = max(dp[i], dp[i * j] + 1);
            }
        } else {
            dp[i] = 0;
        }
    }
    println!("{}", dp.iter().max().unwrap());
}
