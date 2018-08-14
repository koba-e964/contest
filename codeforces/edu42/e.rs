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
    let mut x = vec![0i64; n];
    let mut kind = vec!['+'; n];
    for i in 0 .. n {
        x[i] = get();
        kind[i] = get_word().chars().collect::<Vec<_>>()[0];
    }
    let mut r = Vec::new();
    let mut b = Vec::new();
    let mut p = Vec::new();
    for i in 0 .. n {
        if kind[i] == 'P' {
            p.push((x[i], i));
        }
        if kind[i] != 'B' {
            r.push(x[i]);
        }
        if kind[i] != 'R' {
            b.push(x[i]);
        }
    }
    let mut tot = 0;
    if p.len() >= 1 {
        for i in 0 .. p.len() - 1 {
            let mi = p[i].1;
            let ma = p[i + 1].1;
            let mut mival = 2 * (x[ma] - x[mi]);
            let mut r = Vec::new();
            let mut b = Vec::new();
            let mut r_ma = 0;
            let mut b_ma = 0;
            for i in mi .. ma + 1 {
                if kind[i] != 'B' {
                    r.push(x[i]);
                }
                if kind[i] != 'R' {
                    b.push(x[i]);
                }
            }
            for i in 0 .. r.len() - 1 {
                r_ma = max(r_ma, r[i + 1] - r[i]);
            }
            for i in 0 .. b.len() - 1 {
                b_ma = max(b_ma, b[i + 1] - b[i]);
            }
            mival = min(mival, 3 * (x[ma] - x[mi]) - r_ma - b_ma);
            tot += mival;
        }
        tot += p[0].0 - r[0];
        tot += p[0].0 - b[0];
        tot += r[r.len() - 1] - p[p.len() - 1].0;
        tot += b[b.len() - 1] - p[p.len() - 1].0;
    } else {
        if r.len() >= 1 {
            tot += r[r.len() - 1] - r[0];
        }
        if b.len() >= 1 {
            tot += b[b.len() - 1] - b[0];
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
