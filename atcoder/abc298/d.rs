use std::collections::*;
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

fn main() {
    let q: usize = get();
    let mut que = VecDeque::new();
    que.push_back(1);
    let mut val = 1;
    let inv10 = (3 * MOD + 1) / 10;
    let mut cur = 10;
    for _ in 0..q {
        let ty: i32 = get();
        if ty == 1 {
            let x: i64 = get();
            que.push_back(x);
            val = (10 * val + x) % MOD;
            cur = cur * 10 % MOD;
            continue;
        }
        if ty == 2 {
            let t = que.pop_front().unwrap();
            cur = cur * inv10 % MOD;
            val = (val + MOD - t * cur % MOD) % MOD;
            continue;
        }
        println!("{}", val);
    }
}
