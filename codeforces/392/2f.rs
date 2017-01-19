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

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn main() {
    let n: usize = get();
    let l: i64 = get();
    let r: i64 = get();
    if n == 1 {
        println!("{}", r - l + 1);
        return;
    }
    if n == 2 {
        println!("{}", (r - l + 1) * (r - l));
        return;
    }
    if n >= 25 {
        println!("0");
        return;
    }
    // Find all (s, t, c) s.t. s^{n-1}c, t^{n-1}c are in [l, r].
    let mut tot = 0;
    let mut b: i64 = 1; // b^(n - 1) <= r
    loop {
        let mut cur: i64 = 1;
        for _ in 1 .. n { cur *= b; }
        if cur > r {
            b -= 1;
            break;
        }
        b += 1;
    }
    //println!("b = {}, n - 1 = {}, r = {}", b, n - 1, r);
    for s in 1 .. (b + 1) {
        for t in (s + 1) .. (b + 1) {
            if gcd(s, t) != 1 { continue; }
            let mut pwt = 1;
            let mut pws = 1;
            for _ in 1 .. n { pwt *= t; pws *= s; }
            /*
            for c in (l + pws - 1) / pws .. r / pwt + 1 {
                // println!("s = {}, t = {}, pwt = {}, c = {}", s, t, pwt, c);
                tot += 2;
            }
             */
            tot += 2 * max(0, r / pwt + 1 - (l + pws - 1) / pws);
        }
    }
    
    println!("{}", tot);
}
