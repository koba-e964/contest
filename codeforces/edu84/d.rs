#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Read, Write, BufWriter};

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
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
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    let t: usize = get();
    for _ in 0..t {
        let n: usize = get();
        let p: Vec<usize> = (0..n).map(|_| get::<usize>() - 1).collect();
        let c: Vec<usize> = (0..n).map(|_| get::<usize>() - 1).collect();
        let mut vis = vec![false; n];
        let mut mi = n;
        for i in 0..n {
            if vis[i] { continue; }
            let mut cs = vec![c[i]];
            let mut v = p[i];
            vis[i] = true;
            while v != i {
                vis[v] = true;
                cs.push(c[v]);
                v = p[v];
            }
            let l = cs.len();
            for d in 1..l + 1 {
                if l % d != 0 { continue; }
                for j in 0..d {
                    let ok = (0..l / d).all(|i| cs[i * d + j] == cs[j]);
                    if ok {
                        mi = min(mi, d);
                    }
                }
            }
        }
        puts!("{}\n", mi);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
