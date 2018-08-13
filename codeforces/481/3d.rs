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

const INF: i64 = 1 << 50;

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    let n = get();
    let b: Vec<i64> = (0 .. n).map(|_| get()).collect();
    if n == 1 {
        puts!("0\n");
        return;
    }
    let mut mi = INF;
    for d0 in -1 .. 2 {
        for d1 in -1 .. 2 {
            let mut conf = vec![0; n];
            conf[0] = b[0] + d0;
            conf[1] = b[1] + d1;
            for i in 2 .. n {
                conf[i] = 2 * conf[i - 1] - conf[i - 2];
            }
            let mut ok = true;
            let mut alt = 0;
            for i in 0 .. n {
                if (conf[i] - b[i]).abs() > 1 { ok = false; break; }
                alt += (conf[i] - b[i]).abs();
            }
            if ok {
                mi = min(mi, alt);
            }
        }
    }
    puts!("{}\n", if mi >= INF { -1 } else { mi });
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
