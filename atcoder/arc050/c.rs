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

fn pow(b: i64, n: i64, md: i64) -> i64 {
    let mut sum = 1;
    let mut cur = b % md;
    let mut e = n;
    while e > 0 {
        if e % 2 == 1 {
            sum = sum * cur % md;
        }
        cur = cur * cur % md;
        e /= 2;
    }
    return sum;
}

// 1 + b + ... + b^(n - 1) mod md
fn acc(b: i64, n: i64, md: i64) -> i64 {
    if n <= 0 {
        return 0;
    }
    if n % 2 == 1 {
        return (1 + ((b + b * b) % md) * acc(b * b % md, n / 2, md)) % md;
    }
    return acc(b * b % md, n / 2, md) * (1 + b) % md;
}
fn main() {
    let a: i64 = parse(&getword());
    let b: i64 = parse(&getword());
    let m: i64 = parse(&getword());
    let g = gcd(a, b);
    let t = acc(pow(10, g, m), a / g, m);
    let u = acc(10, b, m);
    println!("{}", t * u % m);
}
