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
    let n: usize = get();
    let mut c: Vec<i64> = (0..n).map(|_| get()).collect();
    let q: usize = get();
    let mut odd_bias = 0;
    let mut even_bias = 0;
    const INF: i64 = 1 << 50;
    let mut odd_mi = INF;
    let mut even_mi = INF;
    for i in 0..n {
        if i % 2 == 0 {
            odd_mi = min(odd_mi, c[i]);
        } else {
            even_mi = min(even_mi, c[i]);
        }
    }
    let mut tot = 0;
    for _ in 0..q {
        let ty: i32 = get();
        match ty {
            1 => {
                let x: usize = get::<usize>() - 1;
                let a: i64 = get();
                if x % 2 == 0 {
                    if c[x] < a + odd_bias {
                        continue;
                    }
                    c[x] -= a;
                    odd_mi = min(odd_mi, c[x]);
                } else {
                    if c[x] < a + even_bias {
                        continue;
                    }
                    c[x] -= a;
                    even_mi = min(even_mi, c[x]);
                }
                tot += a;
            }
            2 => {
                let a: i64 = get();
                if odd_mi < odd_bias + a {
                    continue;
                }
                odd_bias += a;
                tot += ((n + 1) / 2) as i64 * a;
            }
            3 => {
                let a: i64 = get();
                if odd_mi < odd_bias + a || even_mi < even_bias + a {
                    continue;
                }
                odd_bias += a;
                even_bias += a;
                tot += a * n as i64;
            }
            _ => unreachable!(),
        }
    }
    puts!("{}\n", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
