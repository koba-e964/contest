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
    let mut pass = 0;
    let mut fail = std::cmp::min(a, 1 << ((60 + n - 1) / n)) + 1;
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

fn main() {
    let n: i64 = get();
    let mut div = vec![];
    for s in 1..n + 1 {
        if n * n % s == 0 {
            div.push(s);
            if s != n {
                div.push(n * n / s);
            }
        }
    }
    div.sort();
    let m = div.len();
    let mut ans = 0;
    for i in 0..m {
        let s = div[i];
        if s >= 2 * n { continue; }
        for j in i..m {
            if div[j] % s != 0 {
                continue;
            }
            let sa = div[j] / s;
            if sa >= s { continue; }
            let a = s - sa;
            let t = 2 * s - a; // b + c
            let bc = n * n / div[j] + t * s - s * s; // b * c
            let d = t * t - 4 * bc;
            let d2 = nth(d, 2);
            if d2 * d2 != d { continue; }
            let b = (t - d2) / 2;
            if a > b { continue; }
            ans += 1;
        }
    }
    println!("{}", ans);
}
