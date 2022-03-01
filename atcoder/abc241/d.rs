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
    let q: usize = get();
    let mut a = BTreeSet::new();
    for i in 0..q {
        let ty: i32 = get();
        let x: i64 = get();
        if ty == 1 {
            a.insert((x, i));
        } else {
            let k: usize = get();
            if ty == 3 {
                let ans = a.range((x, 0)..).nth(k - 1);
                println!("{}", ans.unwrap_or(&(-1, 0)).0);
            } else {
                let ans = a.range(..(x, q)).rev().nth(k - 1);
                println!("{}", ans.unwrap_or(&(-1, 0)).0);
            }
        }
    }
}
