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

// https://yukicoder.me/problems/no/1573 (3)
// \sum_{1 <= j <= m} j q(q+3)/2 where q = floor(n/j) が答え。
// Tags: floor-of-quotients
fn main() {
    let n: i64 = get();
    let m: i64 = get();
    const W: i64 = 32_000;
    const MOD: i64 = 998_244_353;
    let mut ans = 0;
    for q in 1..W + 1 {
        let qq = q * (q + 3) / 2 % MOD;
        let r = min(m, n / q);
        let l = n / (q + 1);
        if l < r {
            let sum = (r - l) * (r + l + 1) / 2 % MOD;
            ans = (ans + qq * sum) % MOD;
        }
    }
    for j in 1..=n / (W + 1) {
        let q = n / j;
        let qq = q * (q + 3) / 2 % MOD;
        ans = (ans + qq * j) % MOD;
    }
    println!("{}", ans);
}
