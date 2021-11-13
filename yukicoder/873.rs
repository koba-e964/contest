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

// https://yukicoder.me/problems/no/873 (3)
// ナップサックで最小の長さを求める。ただし使用する優先度は 1 > 3 > 5 > ... > 6 > 4 > 2 // である。-> そうでもない。偶数を使った後は優先順位が逆転する。
// Tags: knapsack
fn main() {
    let n: usize = get();
    const INF: usize = 1 << 20;
    let mut dp = vec![(INF, 0); n + 1];
    let mut dpe = vec![(INF, 0); n + 1];
    dp[0] = (0, 0);
    for i in 1..275 {
        let v = 2 * i;
        for j in v * v..n + 1 {
            let (p, _) = dp[j - v * v];
            dp[j] = min(dp[j], (p + v, v));
        }
    }
    for i in (0..275).rev() {
        let v = 2 * i + 1;
        for j in v * v..n + 1 {
            let (p, _) = dp[j - v * v];
            dp[j] = min(dp[j], (p + v, v));
        }
    }
    dpe[0] = (0, 0);
    for i in (1..275).rev() {
        let v = 2 * i;
        for j in v * v..n + 1 {
            let (p, _) = dpe[j - v * v];
            dpe[j] = min(dpe[j], (p + v, v));
        }
    }
    let mut path = vec![];
    let mut cur = n;
    let mut even = false;
    while cur > 0 {
        let t = if even { dpe[cur].1 } else { dp[cur].1 };
        if even {
            assert_eq!(dpe[cur].0, dp[cur].0);
        }
        path.push(t);
        cur -= t * t;
        if t % 2 == 0 {
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
