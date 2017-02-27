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
    let t: Vec<char> = get_word().chars().collect();
    let p: Vec<char> = get_word().chars().collect();
    let n = t.len();
    let m = p.len();
    let a: Vec<_> = (0 .. n).map(|_| get::<usize>() - 1).collect();
    let mut lo = 0;
    let mut hi = n + 1 - m;
    while hi - lo > 1 {
        let mid = (hi + lo) / 2;

        let mut bitmap = vec![true; n];
        for &v in a[0 .. mid].iter() {
            bitmap[v] = false;
        }
        let mut del = Vec::new();
        for i in 0 .. n {
            if bitmap[i] {
                del.push(t[i]);
            }
        }

        let mut cnt = 0;
        for c in del {
            if cnt >= m {
                break;
            }
            if p[cnt] == c {
                cnt += 1;
            }
        }

        if cnt < m {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    println!("{}", lo);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
