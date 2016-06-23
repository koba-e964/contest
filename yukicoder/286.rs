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
            if res.is_err() || u8b[0] <= ' ' as u8 {
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
    let n: usize = get();
    let m: Vec<i32> = (0 .. n).map(|_| get()).collect();
    let mut disc: Vec<i32> = vec![0; 1 << n];
    for bits in 0 .. 1 << n {
        let mut sum = 0;
        for i in 0 .. n {
            if bits & 1 << i != 0 {
                sum += m[i];
            }
        }
        disc[bits] = sum % 1000;
    }
    let mut dp: Vec<i32> = vec![0; 1 << n];
    for bits in 1 .. 1 << n {
        let mut mi = 0x3fff_ffff;
        for i in 0 .. n {
            if bits & (1 << i) != 0 {
                let rest = bits ^ (1 << i);
                mi = min(mi, dp[rest] + max(0, m[i] - disc[rest]));
            }
        }
        dp[bits] = mi;
    }
    println!("{}", dp[(1 << n) - 1]);
    
}
