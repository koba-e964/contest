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

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn main() {
    let a: i64 = get();
    let b: i64 = get();
    let c: i64 = get();
    let d: i64 = get();
    let e: i64 = get();
    let f: i64 = get();
    let x: i64 = get();
    let calc = |a: i64, b: i64, c: i64| {
        let q = x / (a + c);
        a * b * q + b * min(x % (a + c), a)
    };
    println!("{}", match calc(a, b, c).cmp(&calc(d, e, f)) {
        Ordering::Less => "Aoki",
        Ordering::Equal => "Draw",
        Ordering::Greater => "Takahashi",
    });
}
