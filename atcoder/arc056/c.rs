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
    let n: usize = get();
    let k: i64 = get();
    let w: Vec<Vec<i64>> = (0 .. n).map(|_| (0 .. n).map(|_| get()).collect())
        .collect();
 
    let mut cliq = vec![-1_i64; 1 << n];
    cliq[0] = 0;
    for bits in 0 .. (1 << n) {
        let mut sum = 0;
        for i in 0 .. n {
            for j in (i + 1) .. n {
                if bits & (1 << i) != 0 && bits & (1 << j) != 0 {
                    sum += w[i][j];
                }
            }
        }
        cliq[bits] = sum;
    }
 
    let mut dp = vec![-1_i64; 1 << n];
    dp[0] = 0;
    for bits in 1 .. 1 << n {
        let mut sub = bits;
        while sub > 0 {
            if sub & bits == sub { // sub is a subset
                dp[bits] = max(dp[bits], dp[bits - sub] + cliq[sub] + k);
            }
            sub = (sub - 1) & bits;
        }
    }
    // O(3^n)
    println!("{}", dp[(1 << n) - 1] - cliq[(1 << n) - 1]);
}
