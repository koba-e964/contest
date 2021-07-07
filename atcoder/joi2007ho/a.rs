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
    let n: usize = get();
    let k: usize = get();
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        let x: i32 = get();
        acc[i + 1] = acc[i] + x;
    }
    let mut ma = acc[k];
    for i in 0..n - k + 1 {
        ma = max(ma, acc[i + k] - acc[i]);
    }
    println!("{}", ma);
}
