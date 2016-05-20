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

fn kadomatsu(a: i32, b: i32, c: i32) -> bool {
    a != b && b != c && c != a && !(a < b && b < c) && !(a > b && b > c)
}

fn main() {
    let a1: i32 = get();
    let a2: i32 = get();
    let a3: i32 = get();
    let mut cnt = 0;
    for p in 1 .. 1001 {
        if kadomatsu(a1 % p, a2 % p, a3 % p) {
            cnt += 1;
        }
    }
    if kadomatsu(a1, a2, a3) {
        println!("INF");
    } else {
        println!("{}", cnt);
    }
}
