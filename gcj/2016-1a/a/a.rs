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
    let mut stdin = std::io::stdin();
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

fn calc(s: &[char], memo: &mut [Option<Vec<char>>]) -> Vec<char> {
    let n = s.len();
    if n == 0 {
        return Vec::new();
    }
    if let Some(ref v) = memo[n] {
        return v.clone();
    }
    let mut ma = Vec::new();
    let mut machar = ' ';
    for &c in s.iter() {
        machar = max(machar, c);
    }
    for i in 0 .. n {
        if s[i] == machar {
            let mut sub1 = calc(&s[0 .. i], memo);
            let mut res = vec![machar];
            res.append(&mut sub1);
            res.extend_from_slice(&s[i + 1 .. n]);
            ma = max(ma, res);
        }
    }
    memo[n] = Some(ma.clone());
    ma
}

fn solve() {
    let t: usize = get();
    for test_nr in 1 .. t + 1 {
        let s: Vec<_> = get_word().chars().collect();
        let n = s.len();
        let mut memo = vec![None; n + 1];
        println!("Case #{}: {}", test_nr, calc(&s, &mut memo).into_iter()
                 .collect::<String>());
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
