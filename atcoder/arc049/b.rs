#[allow(unused_imports)]
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

fn max(a: f64, b: f64) -> f64 {
    if a < b {
        b
    } else {
        a
    }
}

fn ok(xyc: &Vec<(f64, f64, f64)>, time: f64) -> bool {
    let mut xlo = -1e9_f64;
    let mut xhi = 1e9_f64;
    let mut ylo = -1e9_f64;
    let mut yhi = 1e9_f64;
    let n = xyc.len();
    for i in 0 .. n {
        let (x, y, c) = xyc[i];
        let dist: f64 = time / c;
        xlo = max(xlo, x - dist);
        ylo = max(ylo, y - dist);
        xhi = -max(-xhi, -(x + dist));
        yhi = -max(-yhi, -(y + dist));
    }
    return xlo <= xhi && ylo <= yhi;
}

fn main() {
    let n = parse(&getword());
    let mut xyc = vec![(0.0f64, 0.0f64, 0.0f64); n];
    for r in xyc.iter_mut() {
        let x = parse(&getword());
        let y = parse(&getword());
        let c = parse(&getword());
        *r = (x, y, c);
    }
    let mut hi = 2.5e9_f64;
    let mut lo = 0_f64;
    for _ in 0 .. 100 {
        let mid = (hi + lo) / 2.0;
        if ok(&xyc, mid) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    println!("{}", hi);
}
