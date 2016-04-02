#[allow(unused_imports)]
use std::cmp::*;
use std::io::*;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}
#[allow(dead_code)]
fn getword() -> String {
    let mut stdin = std::io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.is_err() ||u8b[0] <= ' ' as u8 {
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
#[allow(dead_code)]
fn parse<T : std::str::FromStr>(s : &str) -> T {
     return s.parse::<T>().ok().unwrap();
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
fn diff(a: i64, b: i64) -> (i64, i64) {
    let n = b - a;
    let d = b * a;
    (n / gcd(n, d), d / gcd(n, d))
}

fn main() {
    let t1: i64 = parse(&getword());
    let t2: i64 = parse(&getword());
    let t3: i64 = parse(&getword());
    let (n1, d1) = diff(t1, t2);
    let (n2, d2) = diff(t2, t3);
    let n = gcd(n1, n2);
    let d = d1 / gcd(d1, d2) * d2;
    println!("{}/{}", d, n);
    
}
