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

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn calc(x: i64, y: i64, a: i64, b: i64) -> i64 {
    (x - a).max(0) * (y - b).max(0)
}

// https://yukicoder.me/problems/no/2847 (3.5)
// a^2 + b^2 が平方数となる (a, b) 全てに対して、(2 * a, 2 * b) だけ離れた点を数える必要がある。
// a^2 + b^2 が平方数となる (a, b) の列挙は、1+i や有理整数で割り切れないガウス整数を列挙することで行える。
// Tags: enumerating-gaussian-integers-with-square-norm
fn main() {
    let x: i64 = get();
    let y: i64 = get();
    let m: i64 = get();
    let mut tot = 0;
    for a in 1..=x / 2 {
        tot += calc(x, y, 2 * a, 0);
        tot %= m;
    }
    for a in 1..=y / 2 {
        tot += calc(x, y, 0, 2 * a);
        tot %= m;
    }
    for a in 1..2001 {
        for b in 1..2001 {
            if (a + b) % 2 == 0 {
                continue;
            }
            if gcd(a, b) != 1 {
                continue;
            }
            let c = a * a - b * b;
            let d = 2 * a * b;
            let (c, d) = if c < 0 {
                (d, -c)
            } else {
                (c, d)
            };
            for s in 1..=(x / 2 / c).min(y / 2 / d) {
                tot += calc(x, y, c * 2 * s, d * 2 * s) * 2;
                tot %= m;
            }
        }
    }
    println!("{}", tot * 2 % m);
}
