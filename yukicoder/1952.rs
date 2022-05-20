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

const MOD: i64 = 998_244_353;

fn powmod(x: i64, mut e: i64) -> i64 {
    let mut sum = 1;
    let mut cur = x % MOD;
    while e > 0 {
        if e % 2 != 0 {
            sum = sum * cur % MOD;
        }
        cur = cur * cur % MOD;
        e /= 2;
    }
    sum
}

fn main() {
    let n: i64 = get();
    let m: i64 = get();
    if m < 30 {
        let mut x = 0;
        for i in 0..m {
            x ^= n << i;
        }
        println!("{}", x % MOD);
        return;
    }
    let mut x = 0;
    for i in 0..30 {
        x ^= n << i;
    }
    let mut tot = x & 0x3fffffff;
    if n.count_ones() % 2 != 0 {
        tot += powmod(2, m);
        tot += MOD - powmod(2, 30);
    }
    tot += (x >> 30) % MOD * powmod(2, m);
    println!("{}", tot % MOD);
}
