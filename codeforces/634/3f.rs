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

fn calc(to: &[usize], blk: &[bool]) -> (usize, usize) {
    let k = to.len();
    let mut dep = vec![0; k];
    let mut vis = vec![false; k];
    let mut cyc = vec![(0, 0); k];
    let mut path = vec![];
    let mut acc = vec![vec![]; k];
    for i in 0..k {
        let mut v = i;
        if vis[v] { continue; }
        path.clear();
        while !vis[v] {
            vis[v] = true;
            path.push(v);
            v = to[v];
        }
        if cyc[v].0 == 0 {
            let mut back = 0;
            for i in (0..path.len()).rev() {
                if path[i] == v {
                    back = i;
                    break;
                }
            }
            let len = path.len() - back;
            let repr = path[path.len() - 1];
            acc[repr] = vec![0; len];
            for i in 0..path.len() {
                cyc[path[i]] = (len, repr);
                dep[path[i]] = (path.len() - i - 1) % len;
            }
        } else {
            let st = dep[v];
            let lenrepr = cyc[v];
            for i in 0..path.len() {
                cyc[path[i]] = lenrepr;
                dep[path[i]] = (st + path.len() - i) % lenrepr.0;
            }
        }
    }
    for i in 0..k {
        let (_len, repr) = cyc[i];
        let cls = dep[i];
        let num = if blk[i] { 2 } else { 1 };
        acc[repr][cls] = max(acc[repr][cls], num);
    }
    let mut a1 = 0;
    let mut a2 = 0;
    for i in 0..k {
        for j in 0..acc[i].len() {
            match acc[i][j] {
                0 => (),
                1 => a1 += 1,
                2 => {
                    a1 += 1;
                    a2 += 1;
                }
                _ => panic!(),
            }
        }
    }
    (a1, a2)
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    let t: usize = get();
    for _ in 0..t {
        let n: usize = get();
        let m: usize = get();
        let c: Vec<Vec<_>>
            = (0..n).map(|_| get_word().chars().collect()).collect();
        let s: Vec<Vec<_>>
            = (0..n).map(|_| get_word().chars().collect()).collect();
        let k = n * m;
        let mut to = vec![0; k];
        let mut blk = vec![false; k];
        for i in 0..n {
            for j in 0..m {
                let v = i * m + j;
                let tot = match s[i][j] {
                    'R' => v + 1,
                    'L' => v - 1,
                    'U' => v - m,
                    'D' => v + m,
                    _ => panic!(),
                };
                to[v] = tot;
                blk[v] = c[i][j] == '0';
            }
        }
        let (a1, a2) = calc(&to, &blk);
        puts!("{} {}\n", a1, a2);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
