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

const THRESHOLD: i64 = 60;

fn powmod(x: i64, mut e: i64, m: i64) -> i64 {
    let mut sum = 1;
    let mut cur = x % m;
    while e > 0 {
        if e % 2 != 0 {
            sum = sum * cur % m;
        }
        cur = cur * cur % m;
        e /= 2;
    }
    sum
}

fn powmod_thresh(x: i64, e: i64, mo: i64) -> i64 {
    let least = (THRESHOLD + mo - 1) / mo * mo;
    if x == 0 {
        return if e == 0 { 1 } else { 0 };
    }
    if x == 1 {
        return 1;
    }
    if e <= 7 {
        let mut v = 1;
        for _ in 0..e { v *= x; }
        return if v >= least { (v - least) % mo + least } else { v };
    }
    let v = powmod(x, e, mo);
    if v < THRESHOLD {
        v + least
    } else {
        v
    }
}

fn main() {
    let a: i64 = get();
    let b: i64 = get();
    let c: i64 = get();
    let k = powmod_thresh(b, c, 4);
    println!("{}", powmod(a, k, 10));
}
