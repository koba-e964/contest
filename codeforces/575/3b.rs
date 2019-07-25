#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Read, BufWriter, Write};

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
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    let q: usize = get();
    for _ in 0..q {
        let n: usize = get();
        let k: usize = get();
        let a: Vec<i64> = (0..n).map(|_| get()).collect();
        let mut acc = vec![0; n + 1];
        for i in 0..n {
            acc[i + 1] = (acc[i] + a[i]) % 2;
        }
        let mut cur = acc[n];
        let mut ev = vec![n];
        for i in (0..n + 1).rev() {
            if cur != acc[i] {
                cur = acc[i];
                ev.push(i);
            }
        }
        ev.reverse();
        if ev.len() <= k || (acc[n] + (k as i64)) % 2 != 0 {
            puts!("NO\n");
        } else {
            puts!("YES\n");
            for i in 0..k {
                puts!("{}{}", ev[ev.len() - k + i], if i + 1 == k { "\n" } else { " " });
            }
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
