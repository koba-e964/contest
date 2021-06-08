#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
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

const EPS: f64 = 1.0e-8;

fn normalize(mut f: f64) -> f64 {
    let pi = std::f64::consts::PI;
    if f < -EPS {
        f += 2.0 * pi;
    }
    if f >= 2.0 * pi - EPS {
        f -= 2.0 * pi;
    }
    f
}

fn main() {
    loop {
        let n: usize = get();
        let w: f64 = get();
        if n == 0 {
            break;
        }
        let xy: Vec<(f64, f64)> = (0..n).map(|_| (get(), get())).collect();
        let mut ev = vec![];
        let mut cur = 0;
        for i in 0..n {
            let (x, y) = xy[i];
            if w * w >= x * x + y * y {
                cur += 1;
            } else {
                let theta = y.atan2(x);
                let delta = (w / (x * x + y * y).sqrt()).asin();
                ev.push((normalize(theta - delta), 1));
                ev.push((normalize(theta + delta), 0));
                if xy[i].1.abs() <= w && xy[i].1 < w && xy[i].0 >= 0.0 {
                    cur += 1;
                }
            }
        }
        ev.sort_by(|&(x, _), &(y, _)| x.partial_cmp(&y).unwrap());
        let mut ma = cur;
        for &(_, ty) in &ev {
            if ty == 1 {
                cur += 1;
            } else {
                cur -= 1;
            }
            ma = max(ma, cur);
        }
        println!("{}", ma);
    }
}
