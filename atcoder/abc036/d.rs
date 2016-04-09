#[allow(unused_imports)]
use std::cmp::*;
use std::io::*;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}
#[allow(dead_code)]
fn getword() -> String {
    let mut stdin = std::io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.is_err() ||u8b[0] <= ' ' as u8 {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = std::string::String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}
#[allow(dead_code)]
fn parse<T : std::str::FromStr>(s : &str) -> T {
     return s.parse::<T>().ok().unwrap();
}

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T {
    parse(&getword())
}

const MD: i64 = 1_000_000_007;

fn rec(n: usize, e: &[Vec<usize>], v: usize, bef: usize) -> (i64, i64) {
    let mut ma1: i64 = 1;
    let mut ma2: i64 = 1;
    for u in e[v].iter().cloned() {
        if u == bef {
            continue;
        }
        let (sub1, sub2) = rec(n, e, u, v);
        ma2 *= sub1;
        ma2 %= MD;
        ma1 *= sub2;
        ma1 %= MD;
    }
    return ((ma1 + ma2) % MD, ma2);
}

fn main() {
    let n: usize = get();
    let mut e: Vec<Vec<usize> > = vec![Vec::new(); n];
    for _ in 0 .. (n - 1) {
        let a = get::<usize>();
        let b = get::<usize>();
        e[a - 1].push(b - 1);
        e[b - 1].push(a - 1);
    }
    println!("{}", rec(n, &e, 0, n).0);
}
