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

fn calc(p: f64, x: f64) -> f64 {
    x + p * 0.5_f64.powf(x / 1.5)
}

fn main() {
    let p: f64 = get();
    let mut lo = 0.0_f64;
    let mut hi = 1.0e5_f64;
    for _ in 0 .. 200 {
        let mi1 = (2.0 * lo + hi) / 3.0;
        let mi2 = (lo + 2.0 * hi) / 3.0;
        if calc(p, mi1) < calc(p, mi2) {
            hi = mi2;
        } else {
            lo = mi1;
        }
    }
    println!("{}", calc(p, lo));
}
