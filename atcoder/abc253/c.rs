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

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn main() {
    let q: u32 = get();
    let mut bs = BTreeSet::new();
    for time in 0..q {
        let ty: i32 = get();
        if ty == 1 {
            let x: i32 = get();
            bs.insert((x, time));
        } else if ty == 2 {
            let x: i32 = get();
            let mut c: i64 = get();
            while c > 0 {
                if let Some(&v) = bs.range((x, 0)..).next() {
                    if v.0 == x {
                        bs.remove(&v);
                    } else {
                        break;
                    }
                } else {
                    break;
                }
                c -= 1;
            }
        } else {
            let &mi = bs.iter().next().unwrap();
            let &ma = bs.iter().rev().next().unwrap();
            println!("{}", ma.0 - mi.0);
        }
    }
}
