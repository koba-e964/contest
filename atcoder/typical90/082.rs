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

const MOD: i64 = 1_000_000_007;

fn sum(x: i64) -> i64 {
    x % MOD
        * ((x - 1) % MOD) % MOD
        * (MOD + 1) / 2 % MOD
}

fn calc(x: i64) -> i64 {
    let mut c = 1;
    let mut tot = 0;
    for k in 0..18 {
        let l = c;
        let r = min(10 * c, x);
        if l < r {
            let val = (sum(r) - sum(l) + MOD) % MOD;
            tot += val * (k + 1);
        }
        c *= 10;
    }
    if c < x {
        let val = (sum(x) - sum(c) + MOD) % MOD;
        tot += val * 19;
    }
    tot % MOD
}

fn main() {
    let l: i64 = get();
    let r: i64 = get();
    println!("{}", (calc(r + 1) - calc(l) + MOD) % MOD);
}
