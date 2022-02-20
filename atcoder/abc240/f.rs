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

const INF: i64 = 1 << 50;

fn calc(xy: &[(i64, i64)]) -> i64 {
    let n = xy.len();
    let mut ans = -INF;
    let mut d = 0;
    let mut e = 0;
    for i in 0..n {
        let (x, y) = xy[i];
        let mut tmp = e + d + x;
        if d + x >= 0 && d + x * y < 0 {
            let q = d.abs() / x.abs();
            if q > 0 {
                tmp = max(tmp, e + d * q + x * q * (q + 1) / 2);
            }
        }
        e += d * y + x * ((y + 1) * y / 2);
        d += x * y;
        tmp = max(tmp, e);
        ans = max(ans, tmp);
    }
    ans
}

fn main() {
    let t: usize = get();
    for _ in 0..t {
        let n: usize = get();
        let _m: i64 = get();
        let xy: Vec<_> = (0..n).map(|_| {
            let x: i64 = get();
            let y: i64 = get();
            (x, y)
        }).collect();
        println!("{}", calc(&xy));
    }
}
