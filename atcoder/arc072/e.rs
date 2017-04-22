#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Read;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}
fn get_word() -> String {
    let mut stdin = std::io::stdin();
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

const MAX: i64 = 1 << 30;

fn compact(a: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut ret = Vec::new();
    let mut lo = 0;
    let mut hi = -1;
    for (x, y) in a {
        if hi == -1 || hi <= x - 2 {
            if hi >= 0 {
                ret.push((lo, hi));
            }
            lo = x;
            hi = y;
        } else {
            hi = y;
        }
    }
    if hi >= 0 {
        ret.push((lo, hi));
    }
    ret
}

fn trans(rcur: i64, b: i64) -> i64 {
    if b >= 2 * (rcur + 1) {
        return rcur;
    }
    return b + rcur;
}

fn solve() {
    let n = get();
    let dd: i64 = get();
    let d: Vec<i64> = (0 .. n).map(|_| get()).collect();
    let qn = get();
    let q: Vec<usize> = (0 .. qn).map(|_| get()).collect();
    let mut dsim = vec![dd; n + 1];
    for i in 0 .. n {
        let c = dsim[i];
        let x = min((c - d[i]).abs(), c);
        dsim[i + 1] = x;
    }
    let mut rcur = 0;
    let mut poss = vec![false; n];
    for i in (0 .. n).rev() {
        // Can block dsim[i] -> rcur ?
        let mut ok = false;
        let dcur = dsim[i];
        ok = rcur < dcur;
        poss[i] = ok;
        rcur = trans(rcur, d[i]);
    }
    for v in q {
        println!("{}", if poss[v - 1] { "YES" } else { "NO" });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
