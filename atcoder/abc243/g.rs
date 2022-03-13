use std::cmp::*;
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

const B: usize = 60000;

fn calc(x: i64, acc: &[i64]) -> i64 {
    let sq = nth(x, 2);
    let sqsq = nth(sq, 2);
    let mut tot = 0;
    for i in 1..sqsq + 1 {
        tot += acc[i as usize] * (min(sq + 1, (i + 1) * (i + 1)) - i * i);
    }
    tot
}

fn main() {
    let t: usize = get();
    let mut acc = vec![0; B];
    acc[1] = 1;
    for i in 2..B {
        let s = nth(i as i64, 2);
        acc[i] = acc[i - 1] + acc[s as usize];
    }
    for _ in 0..t {
        let x: i64 = get();
        println!("{}", calc(x, &acc));
    }
}
