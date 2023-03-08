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

#[inline(always)]
fn zmod(a: i64, b: i64, p: i64) -> i64 {
    let mut x = a + b;
    if x >= p {
        x -= p;
    }
    x
}

// \sum_{i, j >= 0, ai + bj = n} (a/(ai)!)(b/(bj)!)n!
fn main() {
    let n: usize = get();
    let p: i64 = get();
    let mut fac = vec![0; n + 1];
    let mut invfac = vec![0; n + 1];
    fac[0] = 1;
    for i in 0..n {
        fac[i + 1] = fac[i] * (i + 1) as i64 % p;
    }
    invfac[n] = powmod(fac[n], p - 2, p);
    for i in (0..n).rev() {
        invfac[i] = invfac[i + 1] * (i + 1) as i64 % p;
    }
    let mut combtbl = vec![0; n + 1];
    for i in 0..n + 1 {
        combtbl[i] = fac[n] * invfac[i] % p * invfac[n - i] % p;
    }
    let mut tbl = vec![0; n + 1];
    for l in 1..n + 1 {
        let mut x = 0;
        for i in 1..n / l + 1 {
            x = zmod(x, combtbl[l * i], p);
        }
        tbl[l] = x;
    }
    // sab[l] = \sum {ab | lcm(a, b) = l}
    let mut sab = vec![0; n + 1];
    for i in 1..n + 1 {
        sab[i] = i as i64;
    }
    for i in (1..n + 1).rev() {
        for j in 2..n / i + 1 {
            sab[i * j] = zmod(sab[i * j], sab[i], p);
        }
    }
    sab[0] = n as i64 * (n + 1) as i64 / 2 % p;
    for i in 1..n + 1 {
        sab[i] = sab[i] * invfac[i] % p;
    }
    let mut ans = 0;
    for i in 0..n + 1 {
        ans = (ans + sab[i] * sab[n - i]) % p;
    }
    ans = ans * fac[n] % p;
    println!("{}", ans);
}
