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

fn square(poly: &[i64]) -> Vec<i64> {
    let d = poly.len();
    let mut ret = vec![0; d];
    for i in 0 .. d {
        let mut sum = 0;
        for j in 0 .. (i + 1) {
            sum += poly[j] * poly[i - j];
        }
        ret[i] = sum;
    }
    return ret;
}
fn main() {
    let n: usize = get();
    let mut poly: Vec<i64> = vec![0; 2 * n + 1];
    for i in 0 .. (n + 1) {
        poly[i] = 1;
    }
    for _ in 0 .. 3 {
        poly = square(&poly);
    }
    println!("{}", poly[2 * n]);
}
