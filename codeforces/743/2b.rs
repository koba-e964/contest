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

fn main() {
    let t: usize = get();
    for _ in 0..t {
        let n: usize = get();
        let a: Vec<usize> = (0..n).map(|_| get::<usize>() - 1).collect();
        let b: Vec<usize> = (0..n).map(|_| get::<usize>() - 1).collect();
        let mut ans = 2 * n;
        let mut dp = vec![2 * n + 1; 2 * n];
        for i in 0..n {
            dp[a[i]] = i;
        }
        for i in 1..2 * n {
            dp[i] = min(dp[i], dp[i - 1]);
        }
        for i in 0..n {
            ans = min(ans, dp[b[i]] + i);
        }
        println!("{}", ans);
    }
}
