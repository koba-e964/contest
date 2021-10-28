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

const MOD: i64 = 1_000_000_007;

fn dfs(k: usize, l: i64, r: i64, len: &[i64]) -> (i64, i64) {
    if l == r {
        return (0, 1);
    }
    if k == 1 {
        return (1, 1);
    }
    assert!(r <= len[k]);
    assert!(l <= r);
    if (l, r) == (0, len[k]) {
        let mut a = 0;
        let mut b = 1;
        let mut v = k as i64 * k as i64;
        while v > 0 {
            let mut r = v % 10;
            if r == 0 { r = 10; }
            a += r;
            b = b * r % MOD;
            v /= 10;
        }
        let (c, d) = dfs(k - 1, 0, len[k - 1], len);
        a += 2 * c;
        b = b * d % MOD * d % MOD;
        return (a, b);
    }
    let mut a = 0;
    let mut b = 1;
    let mid1 = len[k - 1];
    let mid2 = len[k] - len[k - 1];
    if l < mid1 {
        let (c, d) = dfs(k - 1, l, min(r, mid1), &len);
        a += c;
        b = b * d % MOD;
    }
    if l < mid2 && r > mid1 {
        let s: Vec<_> = format!("{}", k * k).bytes().collect();
        for i in max(l, mid1) - mid1..min(r, mid2) - mid1 {
            let v = (s[i as usize] - b'0') as i64;
            let v = if v == 0 { 10 } else { v };
            a += v;
            b = b * v % MOD;
        }
    }
    if r > mid2 {
        let (c, d) = dfs(k - 1, max(l, mid2) - mid2, r - mid2, &len);
        a += c;
        b = b * d % MOD;
    }
    (a, b)
}

fn main() {
    let k: usize = get();
    let l: i64 = get();
    let r: i64 = get();
    let mut len = vec![0; 62];
    len[1] = 1;
    for i in 2..62 {
        len[i] = len[i - 1] * 2 + format!("{}", i * i).len() as i64;
    }
    let k = min(k, 61);
    if r > len[k] {
        println!("-1");
        return;
    }
    let (a, b) = dfs(k, l - 1, r, &len);
    println!("{} {}", a, b);
}
