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

// If p10 = 10^k, this function returns a k-digit number
// that is obtained as floor(n * 10^?).
fn pick_digits(mut n: i64, p10: i64) -> i64 {
    if n < p10 / 10 {
        while n < p10 / 10 {
            n *= 10;
        }
        return n;
    }
    while n >= p10 {
        n /= 10;
    }
    n
}

fn main() {
    let t: i32 = get();
    for _ in 0..t {
        let n: i64 = get();
        let k: i64 = get();
        let mut p = 1;
        while p <= n {
            p *= 10;
        }
        let mut ans = 0;
        while p > 1 {
            p /= 10;
            // [p, cur)
            let cur = pick_digits(k, 10 * p);
            let l = p;
            let r = min(cur + 1, min(n + 1, if cur > k { cur } else { cur + 1 }));
            if l < r {
                ans += r - l;
            }
        }
        println!("{}", ans);
    }
}
