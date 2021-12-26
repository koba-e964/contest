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

// https://yukicoder.me/problems/no/1793 (3.5)
// 対数の世界で二分探索する。ただし答えが 10^{-5} 以下であれば 10^{-5} と答えて良い。
fn main() {
    let t: usize = get();
    for _ in 0..t {
        let mut hi = 12.22e74f64.ln();
        let mut lo = 1.0e-6f64.ln();
        for _ in 0..24 {
            let mid = (hi + lo) / 2.0;
            let y = mid.exp();
            println!("? {}", y);
            let s = get_word();
            if s == "Yes" {
                lo = mid;
            } else if s == "No" {
                hi = mid;
            } else {
                panic!();
            }
        }
        println!("! {}", ((lo + hi) / 2.0).exp());
    }
}
