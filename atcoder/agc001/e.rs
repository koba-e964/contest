#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::*;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}
fn get_word() -> String {
    let mut stdin = std::io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.is_err() || res.ok().unwrap() == 0 || u8b[0] <= ' ' as u8 {
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
fn parse<T: std::str::FromStr>(s: &str) -> T { s.parse::<T>().ok().unwrap() }

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { parse(&get_word()) }

const MOD: i64 = 1_000_000_007;
const BIAS: usize = 2010;
const W: usize = 8001;

fn inv(t: i64) -> i64 {
    let mut sum = 1;
    let mut cur = t;
    let mut e = MOD - 2;
    while e > 0 {
        if e % 2 == 1 {
            sum = sum * cur % MOD;
        }
        cur = cur * cur % MOD;
        e /= 2;
    }
    sum
}

fn main() {
    let n: usize = get();
    let mut tbl: Vec<Vec<i32>> = vec![Vec::new(); W];
    let mut inv_tbl = vec![0_i64; W + 1];
    for i in 0 .. (W + 1) {
        inv_tbl[i] = inv(i as i64);
    }
    for i in 0 .. W {
        tbl[i] = vec![0; i + 1];
        let mut t = 1;
        for j in 0 .. (i + 1) {
            tbl[i][j] = t as i32;
            t = t * (i - j) as i64 % MOD;
            t = t * inv_tbl[j + 1] % MOD;
        }
    }
    let mut f = vec![0_i64; 2 * BIAS];
    let mut dbl = 0_i64;
    let mut e: Vec<(usize, usize)> = Vec::new();
    for _ in 0 .. n {
        let a: usize = get();
        let b: usize = get();
        e.push((a + b, BIAS - a));
        dbl += tbl[2 * a + 2 * b][2 * a] as i64;
        dbl %= MOD;
    }
    e.push((0, 0));
    e.sort();
    e.reverse();
    let mut cur = 2 * BIAS;
    for (a, b) in e {
        while cur > a {
            // f *= (1 + x)
            for i in (0 .. 2 * BIAS - 1).rev() {
                f[i + 1] += f[i];
                f[i + 1] %= MOD;
            }
            cur -= 1;
        }
        if a != 0 {
            f[b] += 1;
            f[b] %= MOD;
        }
    }
    // squaring
    let mut f2 = 0;
    for i in 1 .. 2 * BIAS {
        f2 += f[2 * BIAS - i] * f[i];
        f2 %= MOD;
    }
    let mut r = (f2 - dbl + MOD) % MOD;
    r *= (MOD + 1) / 2;
    r %= MOD;
    println!("{}", r);
}
