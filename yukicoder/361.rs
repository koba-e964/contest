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
    let l: usize = get();
    let d: usize = get();
    let mut dp = vec![0_i64; l + 1]; // Grundy numbers
    for i in 1 .. (l + 1) {
        let mut occ = HashSet::new();
        for x in 1 .. (i - 1) {
            for y in (x + 1) .. (i - x) {
                let z = i - x - y;
                if z <= y || z - x > d {
                    continue;
                }
                occ.insert(dp[x] ^ dp[y] ^ dp[z]);
            }
        }
        // calculate mex
        let mut cur = 0;
        loop {
            if occ.contains(&cur) {
                cur += 1;
            } else {
                break;
            }
        }
        dp[i] = cur;
    }
    println!("{}", if dp[l] >= 1 { "kado" } else { "matsu" }); 
}
