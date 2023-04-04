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

fn main() {
    let n: i64 = get();
    let m: i64 = get();
    if n.saturating_mul(n) < m {
        println!("-1");
        return;
    }
    let mut ans = m + n;
    for i in 1..1_500_000 {
        let q = (m + i - 1) / i;
        if i <= n && q <= n {
            ans = min(ans, q * i);
        }
    }
    println!("{}", ans);
}
