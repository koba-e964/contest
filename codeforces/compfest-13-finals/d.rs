#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Read;

#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

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
    let mut ans = 0;
    for i in 0..4_000_000 {
        let t: Vec<_> = format!("{}", 25 * i).chars().collect();
        if s.len() == t.len() {
            let mut x = 0;
            let mut ok = true;
            for i in 0..s.len() {
                if s[i] != t[i] && s[i] != '_' && s[i] != 'X' {
                    ok = false;
                    break;
                }
                if s[i] == 'X' {
                    let idx = t[i] as u8 - b'0';
                    x |= 1 << idx;
                }
            }
            if ok && (x & -x) == x {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
