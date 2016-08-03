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


const MOD: i64 = 1_000_000_007;

fn powmod(a: i64, b: i64, m: i64) -> i64 {
    let mut sum: i64 = 1;
    let mut cur: i64 = a % m;
    let mut e = b;
    while e > 0 {
        if e % 2 == 1 {
            sum = sum * cur % m;
        }
        cur = cur * cur % m;
        e /= 2;
    }
    sum
}
fn main() {
    let v: Vec<i64> = get_word().split("^").map(|v| parse::<i64>(v)).collect::<Vec<i64>>();
    print!("{} ", powmod(powmod(v[0], v[1], MOD), v[2], MOD));
    let mut e = powmod(v[1], v[2], MOD - 1);
    if v[1] >= 1 && e == 0 {
        e = MOD - 1;
    }
    println!("{}", powmod(v[0], e, MOD));
}
