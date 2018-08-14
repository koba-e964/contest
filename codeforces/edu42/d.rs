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
    let a: Vec<i64> = (0 .. n).map(|_| get()).collect();
    let mut map = BTreeMap::new();
    for i in 0 .. n {
        map.entry(a[i]).or_insert(Vec::new()).push(i);
    }
    let mut decided = Vec::new();
    while let Some(&key) = map.keys().next() {
        let mut v = map.remove(&key).unwrap();
        if v.is_empty() { continue; }
        v.sort();
        if v.len() % 2 == 1 {
            decided.push((v[v.len() - 1], key));
        }
        let targ = map.entry(2 * key).or_insert(Vec::new());
        for i in 0 .. v.len() / 2 {
            targ.push(v[2 * i + 1]);
        }
    }
    decided.sort();
    puts!("{}\n", decided.len());
    for i in 0 .. decided.len() {
        puts!("{}{}", decided[i].1, if i == decided.len() - 1 { "\n" } else { " " });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
