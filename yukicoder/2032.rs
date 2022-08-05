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

// Ported from ACL.
// Computes \sum_{i = 0}^{n - 1} floor((a * i + b) / m).
// Verified by: https://atcoder.jp/contests/arc111/submissions/23877969
fn floor_sum(n: i64, m: i64, a: i64, b: i64) -> i64 {
    fn internal(n: i64, m: i64, mut a: i64, mut b: i64, mut acc: i64) -> i64 {
        if a >= m {
            let q = a / m;
            acc += (n - 1) * n / 2 * q;
            a -= q * m;
        }
        if b >= m {
            let q = b / m;
            acc += n * q;
            b -= q * m;
        }
        let y_max = (a * n + b) / m;
        let x_max = y_max * m - b;
        if y_max == 0 {
            return acc;
        }
        acc += (n - (x_max + a - 1) / a) * y_max;
        let mut sub_b = a - x_max % a;
        if sub_b >= a {
            sub_b -= a;
        }
        internal(y_max, a, m, sub_b, acc)
    }
    internal(n, m, a, b, 0)
}

fn rem_range((a, b): (i64, i64), n: i64, m: i64, r: i64) -> i64 {
    floor_sum(n, m, a, b + m) - floor_sum(n, m, a, b + m - r)
}

fn calc(r: i64, k: i64, c: i64) -> i64 {
    let r = r / k;
    let mut x = 1i64;
    let mut ans = 0;
    for _ in 0..10 {
        ans += rem_range((k, 0), r + 1, 10 * x, (c + 1) * x)
            - rem_range((k, 0), r + 1, 10 * x, c * x);
        x *= 10;
    }
    ans
}

// Solved with hints
fn main() {
    let t: usize = get();
    for _ in 0..t {
        let l: i64 = get();
        let r: i64 = get();
        let k: i64 = get();
        let c: i64 = get();
        println!("{}", calc(r, k, c) - calc(l - 1, k, c));
    }
}
