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

fn powmod(x: i64, mut e: i64, m: i64) -> i64 {
    let mut sum = 1;
    let mut cur = x % m;
    while e > 0 {
        if e % 2 != 0 {
            sum = sum * cur % m;
        }
        cur = cur * cur % m;
        e /= 2;
    }
    sum
}

/**
 * Calculates x s.t. x^2 = a (mod p)
 * p is prime
 * Verified by: CF #395 Div1-C
 *              (http://codeforces.com/contest/763/submission/24380573)
 */
fn modsqrt(mut a: i64, p: i64) -> Option<i64> {
    a %= p;
    if a == 0 {
        return Some(0);
    }
    if p == 2 {
        return Some(a);
    }
    if powmod(a, (p - 1) / 2, p) != 1 {
        return None;
    }
    let mut b = 1;
    while powmod(b, (p - 1) / 2, p) == 1 {
        b += 1;
    }
    let mut e = 0;
    let mut m = p - 1;
    while m % 2 == 0 {
        m /= 2;
        e += 1;
    }
    let mut x = powmod(a, (m - 1) / 2, p);
    let mut y = a * (x * x % p) % p;
    x = x * a % p;
    let mut z = powmod(b, m, p);
    while y != 1 {
        let mut j = 0;
        let mut t = y;
        while t != 1 {
            j += 1;
            t = t * t % p;
        }
        assert!(j < e);
        z = powmod(z, 1 << (e - j - 1), p);
        x = x * z % p;
        z = z * z % p;
        y = y * z % p;
        e = j;
    }
    Some(x)
}

fn mul((a, b): (i64, i64), (c, d): (i64, i64), m: i64) -> (i64, i64) {
    ((a * c + 3 * b * d) % m, (a * d + b * c) % m)
}

fn pow(x: (i64, i64), mut e: i64, m: i64) -> (i64, i64) {
    let mut prod = (1, 0);
    let mut cur = x;
    while e > 0 {
        if e % 2 == 1 {
            prod = mul(prod, cur, m);
        }
        cur = mul(cur, cur, m);
        e /= 2;
    }
    prod
}

fn main() {
    let n: i64 = get();
    let m: i64 = get();
    if m == 2 || m == 3 {
        println!("0");
        return;
    }
    if m % 12 == 1 || m % 12 == 11 {
        let x = modsqrt(3, m).unwrap();
        let exp = powmod(2, n, m - 1);
        let y = powmod(2 + x, exp, m);
        let z = powmod(2 - x + m, exp, m);
        println!("{}", (y + z + m - 2) % m);
        return;
    }
    let exp = powmod(2, n, m + 1);
    let ans = pow((2, 1), exp, m);
    println!("{}", (ans.0 * 2 + m - 2) % m);
}
