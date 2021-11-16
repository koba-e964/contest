use std::cmp::*;
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

// 0 <= x <= n, 0 <= y <= m
fn dfs(n: i64, m: i64, axeq: bool, xoeq: bool, memo: &mut HashMap<(i64, i64, bool, bool), i64>) -> i64 {
    if n == 0 && m == 0 {
        return if axeq && xoeq {
            1
        } else {
            0
        };
    }
    let key = (n, m, axeq, xoeq);
    if let Some(&val) = memo.get(&key) {
        return val;
    }
    let mut ans = 0;
    for a in 0..min(2, n + 1) {
        for b in 0..min(2, m + 1) {
            let naxeq = (a & b, true) <= (a ^ b, axeq);
            let noxeq = (a ^ b, true) <= (a | b, xoeq);
            ans += dfs((n - a) / 2, (m - b) / 2, naxeq, noxeq, memo);
            if ans >= MOD {
                ans -= MOD;
            }
        }
    }
    memo.insert(key, ans);
    ans
}

// Tags: digital-dp
fn main() {
    let n: i64 = get();
    let mut memo = HashMap::new();
    let res = dfs(n, n, false, false, &mut memo);
    let inv2 = (MOD + 1) / 2;
    println!("{}", res * inv2 % MOD);
}
