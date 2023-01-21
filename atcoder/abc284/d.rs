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

fn nth(a: i64, n: i64) -> i64 {
    let mut pass = 0;
    let mut fail = std::cmp::min(a, 1 << ((64 + n - 1) / n)) + 1;
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
    let t: usize = get();
    for _ in 0..t {
        let n: i64 = get();
        let mut p = 0;
        let mut q = 0;
        for i in 2..2100000 {
            if n % i == 0 {
                if n / i % i == 0 {
                    p = i;
                    q = n / i / i;
                } else {
                    q = i;
                    p = nth(n / i, 2);
                }
                break;
            }
        }
        println!("{} {}", p, q);
    }
}
