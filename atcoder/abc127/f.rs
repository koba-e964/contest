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
    let q: usize = get();
    let mut former = BinaryHeap::new();
    let mut latter = BinaryHeap::new();
    let mut formersum = 0;
    let mut lattersum = 0;
    let mut cons = 0;
    for _ in 0..q {
        let ty: i32 = get();
        if ty == 1 {
            // UPD
            let mut a: i64 = get();
            let b: i64 = get();
            if former.len() == 0 {
                former.push(a);
                formersum += a;
            } else if former.len() == latter.len() {
                let mut y: i64 = latter.pop().unwrap();
                y = -y;
                lattersum -= y;
                if a > y {
                    std::mem::swap(&mut a, &mut y);
                }
                former.push(a);
                formersum += a;
                latter.push(-y);
                lattersum += y;
            } else {
                latter.push(-a);
                lattersum += a;
                if let (Some(mut x), Some(mut y)) = (former.pop(), latter.pop()) {
                    y = -y;
                    formersum -= x;
                    lattersum -= y;
                    if x > y {
                        std::mem::swap(&mut x, &mut y);
                    }
                    former.push(x);
                    latter.push(-y);
                    formersum += x;
                    lattersum += y;
                }
            }
            cons += b;
        } else {
            // QUE
            let x = former.peek().unwrap();
            let mut ans = -formersum + lattersum;
            ans += (former.len() as i64 - latter.len() as i64) * x;
            println!("{} {}", x, ans + cons);
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
