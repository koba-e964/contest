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
    let mut s: VecDeque<_> = get_word().chars().collect();
    let q: usize = get();
    let mut rev = false;
    for _ in 0..q {
        let ty: i32 = get();
        if ty == 1 {
            rev = !rev;
        } else {
            let f: i32 = get();
            let c: char = get_word().chars().next().unwrap();
            if (f == 1) ^ rev {
                s.push_front(c);
            } else {
                s.push_back(c);
            }
        }
    }
    let mut s = s.into_iter().collect::<Vec<_>>();
    if rev {
        s.reverse();
    }
    println!("{}", s.into_iter().collect::<String>());
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
