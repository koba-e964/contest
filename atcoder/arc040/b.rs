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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    let n = get();
    let r: usize = get();
    let mut s: Vec<char> = get_word().chars().collect();
    let mut time = 0;
    let mut last = n;
    for i in 0 .. n {
        if s[i] == '.' {
            last = i;
        }
    }
    if last == n {
        puts!("0\n");
        return;
    }
    last = if last >= r - 1 { last - r + 1 } else { 0 };
    // greed
    for i in (0 .. last + r).rev() {
        if s[i] == '.' {
            for j in max(0, i as i32 - r as i32 + 1) as usize .. i + 1  {
                s[j] = 'o';
            }
            time += 1;
        }
    }
    puts!("{}\n", time + last);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
