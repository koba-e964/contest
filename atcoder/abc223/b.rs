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

fn main() {
    let s: Vec<char> = get_word().chars().collect();
    let mut mi = s.clone();
    let mut ma = s.clone();
    let n = s.len();
    for i in 1..n {
        let mut v = s[i..].to_vec();
        v.extend(&s[..i]);
        mi = min(mi, v.clone());
        ma = max(ma, v);
    }
    println!("{}", mi.into_iter().collect::<String>());
    println!("{}", ma.into_iter().collect::<String>());
}
