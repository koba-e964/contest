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

const MOD: i64 = 1_000_000_007;

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

fn main() {
    let n: i32 = get();
    if n <= 1 {
        println!("5");
        return;
    }
    if n == 2 {
        println!("{}", 10000i64 * 10000 * 10000 - 1);
        return;
    }
    if n == 3 {
        println!("{}", 50_000_000 * 2);
        return;
    }
    if n == 4 {
        println!("1");
        return;
    }
    let m = 2_000_000;
    let mut nom = 1;
    let mut den = 1;
    for i in m + 1..2 * m + 2 {
        nom = nom * (i + 1) % MOD;
    }
    for i in 0..m + 1 {
        den = den * (i + 1) % MOD;
    }
    println!("{}", (nom * powmod(den, MOD - 2, MOD) + MOD - 1) % MOD);
}
