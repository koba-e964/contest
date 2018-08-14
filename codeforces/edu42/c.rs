#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Read, Write, BufWriter};
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

fn is_square(s: Vec<i32>) -> bool {
    if s.len() == 0 { return false; }
    if s[0] == 0 { return false; }
    let mut val = 0;
    for u in s {
        val *= 10;
        val += u as i64;
    }
    let mut pass = 0;
    let mut fail = 1 << 16;
    while fail - pass > 1 {
        let mid = (fail + pass) / 2;
        if mid * mid <= val { pass = mid; }
        else { fail = mid; }
    }
    pass * pass == val
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    let n: Vec<_> = get_word().chars().map(|c| c as u8 as i32 - b'0' as i32).collect();
    let len = n.len();
    let mut ans = 1 << 20;
    for bits in 0 .. 1usize << len {
        let mut t = Vec::new();
        for i in 0 .. len {
            if (bits & 1 << i) == 0 {
                t.push(n[i]);
            }
        }
        if is_square(t) {
            ans = min(ans, bits.count_ones() as i32);
        }
    }
    puts!("{}\n", if ans == 1 << 20 { -1 } else { ans });
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
