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

fn nth(a: i64, n: i64) -> i64 {
    if a <= 1 {
        return a;
    }
    let mut pass = 0;
    let mut fail = std::cmp::min(a, 1 << 32) + 1;
    while fail - pass > 1 {
        let mid = (fail + pass) / 2;
        let mut tmp = 1i64;
        for _ in 0..n {
            tmp = tmp.saturating_mul(mid);
        }
        if tmp <= a {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    pass
}

// Solves y = x(a-x) (x>=0)
fn calc(a: i64, y: i64) -> Option<i64> {
    if 4 * y > a * a {
        return None;
    }
    let sd = nth(a * a - 4 * y, 2);
    if sd * sd != a * a - 4 * y {
        return None;
    }
    Some((a - sd) / 2)
}

fn solve(s: i64, t: i64) -> Vec<(i64, i64, i64)> {
    if t % 2 != 0 || 18 * s >= t * t {
        return vec![];
    }
    let t = t / 2;
    if s * s % t != 0 {
        return vec![];
    }
    let q = s * s / t;
    // q = (t-a)(t-b)(a+b-t)
    let mut divs = vec![];
    let mut d = 1;
    while d * d <= q {
        if q % d == 0 {
            divs.push(d);
            if q != d * d {
                divs.push(q / d);
            }
        }
        d += 1;
    }
    let mut ans = vec![];
    for d in divs {
        if d >= t { continue; }
        let a = t - d;
        if let Some(x) = calc(a, q / d) {
            let b = t - x;
            if b <= 0 { continue; }
            let mut v = [a, b, 2 * t - a - b];
            v.sort_unstable();
            if v[2] <= 1_000_000_000 {
                ans.push((v[0], v[1], v[2]));
            }
        }
    }
    ans.sort_unstable();
    ans.dedup();
    ans
}

fn main() {
    let t: usize = get();
    for _ in 0..t {
        let s: i64 = get();
        let t: i64 = get();
        let v = solve(s, t);
        println!("{}", v.len());
        for (a, b, c) in v {
            println!("{} {} {}", a, b, c);
        }
    }
}
