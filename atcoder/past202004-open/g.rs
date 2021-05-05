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

fn solve() {
    let q: usize = get();
    let mut v = VecDeque::new();
    for _ in 0..q {
        let ty: i32 = get();
        if ty == 1 {
            let c: char = get();
            let x: i64 = get();
            let c = (c as u8 - b'a') as usize;
            v.push_back((c, x));
        } else {
            let mut freq = [0; 26];
            let mut d: i64 = get();
            while d > 0 && !v.is_empty() {
                let (c, mut f) = v.pop_front().unwrap();
                let r = min(f, d);
                d -= r;
                f -= r;
                freq[c] += r;
                if f > 0 {
                    v.push_front((c, f));
                }
            }
            let tot = freq.iter().map(|&x| x * x).sum::<i64>();
            println!("{}", tot);
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
