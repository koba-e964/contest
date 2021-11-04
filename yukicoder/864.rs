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

// Precondition: s > 0, n > 0
// #{(x, y) | 1 <= x, y <= n, x + y = s}
fn calc(s: i64, n: i64) -> i64 {
    if s > 2 * n {
        0
    } else if s <= n + 1 {
        s - 1
    } else {
        s - 1 - 2 * (s - n - 1)
    }
}

fn main() {
    let n: i64 = get();
    let k: i64 = get();
    let mut d = 1;
    let mut tot = 0;
    while d * d <= k {
        if k % d == 0 {
            let tmp = calc(d, n) * calc(k / d, n);
            if k == d * d {
                tot += tmp;
            } else {
                tot += 2 * tmp;
            }
        }
        d += 1;
    }
    println!("{}", tot);
}
