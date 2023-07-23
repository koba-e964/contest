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

// https://yukicoder.me/problems/no/873 (3)
// ナップサックで最小の長さを求める。ただし使用する優先度は 1 > 3 > 5 > ... > 6 > 4 > 2
// である。-> そうでもない。偶数を使った後は優先順位が逆転する。
// -> DP の計算ではなく経路復元のときに優先度を考慮すべき。
// Tags: knapsack, path-recovery
fn main() {
    let n: usize = get();
    const INF: usize = 1 << 20;
    let mut dp = vec![INF; n + 1];
    dp[0] = 0;
    for i in 1..551 {
        for j in i * i..n + 1 {
            let p = dp[j - i * i];
            dp[j] = min(dp[j], p + i);
        }
    }
    let mut path = vec![];
    let mut cur = n;
    let mut even = false;
    while cur > 0 {
        let mut now = n + 1;
        if even {
            for i in 1..551 {
                let v = if i <= 275 { 2 * i } else { 2 * (550 - i) + 1 };
                if cur >= v * v && dp[cur - v * v] + v == dp[cur] {
                    now = v;
                    break;
                }
            }
        } else {
            for i in 1..551 {
                let v = if i <= 275 { 2 * i - 1 } else { 2 * (551 - i) };
                if cur >= v * v && dp[cur - v * v] + v == dp[cur] {
                    now = v;
                    break;
                }
            }
        }
        assert!(now <= n);
        path.push(now);
        cur -= now * now;
        if now % 2 == 0 {
            even = !even;
        }
    }
    let mut bias = 0;
    for v in path {
        for i in 0..v {
            print!("{}", (i + bias) % 2);
        }
        bias += v + 1;
    }
    println!();
}
