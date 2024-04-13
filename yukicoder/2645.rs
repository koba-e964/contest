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

// https://yukicoder.me/problems/no/2645 (3.5)
// 式変形すると、\sum_{i=1}^n \sum{j=1}^{floor(n/i)} 1/(ij) である。
// 以下をやる必要がある:
// - b_a := \sum_{j=1}^{a} 1/j を高速に求める
// - sqrt(n) との大小で b_a の加算の方法を変える
// b_a - ln(a + 0.5) は O(1/a^2) のはずなので、それを使って誤差を小さくする
// Tags: sqrt-decomposition, sum-of-divisors, harmonic-series
fn main() {
    const W: usize = 100_000;
    let mut dp = vec![0.0; W];
    for i in 1..W {
        dp[i] = dp[i - 1] + 1.0 / i as f64;
    }
    let f = |x: i64| {
        if x < W as i64 {
            dp[x as usize]
        } else {
            (x as f64 + 0.5).ln() + 0.57721_56649_01532_86060
        }
    };

    let n: i64 = get();
    let mut s = 1;
    while s * s <= n {
        s += 1;
    }
    s -= 1;
    let mut sum = 0.0;
    for i in 1..s + 1 {
        sum += 1.0 / i as f64 * f(n / i);
    }
    for i in 1..s + 1 {
        let lo = max(s, n / (i + 1));
        let hi = n / i;
        if lo < hi {
            sum += (f(hi) - f(lo)) as f64 * f(i);
        }
    }
    println!("{}", sum);
}
