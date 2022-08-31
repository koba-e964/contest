use std::cmp::*;
use std::io::Read;

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn digsum(mut a: i64) -> i64 {
    let mut b = 0;
    while a > 0 {
        b += a % 10;
        a /= 10;
    }
    b
}

fn calc(r: i64, m: usize) -> i64 {
    let mut cur = 1i64;
    let mut tot = 0;
    let mut pw = 1i64;
    for _ in 0..m {
        pw *= 10;
    }
    for _ in 1..min(r, 16) {
        cur *= 5;
        cur %= pw;
        tot += digsum(cur);
    }
    if r <= 16 { return tot; }
    let mut tbl = [0; 1 << 14];
    for i in 0..1 << 14 {
        cur *= 5;
        cur %= pw;
        tbl[i] = digsum(cur);
    }
    for i in 0..1 << 14 {
        let q = (r - 16 + (1 << 14) - 1 - i as i64) >> 14;
        tot += tbl[i] * q;
    }
    tot
}

fn main() {
    let l: i64 = get();
    let r: i64 = get();
    let m: usize = get();
    println!("{}", calc(r + 1, m) - calc(l, m));
}
