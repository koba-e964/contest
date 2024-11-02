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

const MOD: i64 = 998_244_353;

fn triangle(x: i64) -> i64 {
    let x = x % MOD;
    x * (x - 1) / 2 % MOD
}

// Tags: quotient, sqrt-decomposition
fn main() {
    let n: i64 = get();
    let m: i64 = get();
    let mut x = 1;
    while x * x <= m {
        x += 1;
    }
    x -= 1;
    let mut tot = (m % MOD) * (n % MOD) % MOD;
    for i in 1..x + 1 {
        if i <= n {
            let q = m / i;
            tot += MOD - (q * i) % MOD;
        }
    }
    for j in 1..x + 1 {
        let l = x.max(m / (j + 1));
        let r = (m / j).min(n);
        if l < r {
            tot += MOD - ((r - l) % MOD) * j % MOD * ((l + 1) % MOD) % MOD;
            tot += MOD - triangle(r - l) * j % MOD;
        }
    }
    println!("{}", tot % MOD);
}
