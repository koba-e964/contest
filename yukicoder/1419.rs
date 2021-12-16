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

const MOD: i64 = 1_000_000_007;

// https://yukicoder.me/problems/no/1419 (3)
// あり得る offset とその確率は、-2^K+1 + (0 から 2^{K+1} - 2 までの偶数の一様分布に従う変数) である。
fn main() {
    let n: usize = get();
    let k: i64 = get();
    let x = powmod(2, k, n as i64);
    let base = powmod(2, MOD - 1 - k, MOD);
    let x = (n + 1 - x as usize) % n;
    let m = if n % 2 == 0 {
        n / 2
    } else {
        n
    };
    let r = powmod(2, k, m as i64);
    let q = (powmod(2, k, MOD) + MOD - r) * powmod(m as i64, MOD - 2, MOD) % MOD;
    let mut ans = vec![0; n];
    for i in 0..m {
        if i < r as usize {
            ans[(x + 2 * i) % n] = (q + 1) * base % MOD;
        } else {
            ans[(x + 2 * i) % n] = q * base % MOD;
        }
    }
    for i in 0..n {
        println!("{}", ans[i]);
    }
}
