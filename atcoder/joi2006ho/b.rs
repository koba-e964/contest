#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
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
    let n: usize = get();
    let mut s: Vec<char> = get_word().chars().collect();
    for _ in 0..n {
        let mut cur = s[0];
        let mut l = 1;
        let mut t = vec![];
        for i in 1..s.len() {
            if cur != s[i] {
                t.extend(format!("{}", l).chars());
                t.push(cur);
                cur = s[i];
                l = 0;
            }
            l += 1;
        }
        t.extend(format!("{}", l).chars());
        t.push(cur);
        s = t;
    }
    println!("{}", s.into_iter().collect::<String>());
}
