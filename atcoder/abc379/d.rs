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

fn main() {
    let q: i32 = get();
    let mut s = BTreeSet::new();
    let mut bias = 0;
    for idx in 0..q {
        let ty: i32 = get();
        if ty == 1 {
            s.insert((-bias, idx));
        } else if ty == 2 {
            let t: i64 = get();
            bias += t;
        } else {
            let h: i64 = get();
            let tmp = s.range((h - bias, 0)..).cloned().collect::<Vec<_>>();
            println!("{}", tmp.len());
            for t in tmp {
                s.remove(&t);
            }
        }
    }
}
