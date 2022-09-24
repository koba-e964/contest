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

fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn main() {
    let t: i32 = get();
    for _ in 0..t {
        let mut n2: i64 = get();
        let mut n3: i64 = get();
        let mut n4: i64 = get();
        let mut ans = 0;
        let r = min(n4, n3 / 2);
        n3 -= 2 * r;
        n4 -= r;
        ans += r;
        let r = min(n2 / 2, n3 / 2);
        n2 -= 2 * r;
        ans += r;
        ans += min((2 * n4 + n2) / 5, n2);
        println!("{}", ans);
    }
}
