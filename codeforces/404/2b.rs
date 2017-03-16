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

fn two_way_sort(a: &[(i64, i64)]) -> (Vec<(i64, i64)>, Vec<(i64, i64)>) {
    let mut r1 = a.to_vec();
    let mut r2 = a.to_vec();
    r1.sort();
    r2.sort_by_key(|&(x, y)| (y, x));
    (r1, r2)
}

fn solve() {
    let n = get();
    let mut lr1 = Vec::<(i64, i64)>::new();
    for _ in 0 .. n {
        let l = get();
        let r = get();
        lr1.push((l, r));
    }
    let m = get();
    let mut lr2 = Vec::<(i64, i64)>::new();
    for _ in 0 .. m {
        let l = get();
        let r = get();
        lr2.push((l, r));
    }
    let (a1, b1) = two_way_sort(&lr1);
    let (a2, b2) = two_way_sort(&lr2);
    let ma = 0;
    let tmp = max(a1[n - 1].0 - b2[0].1, 0);
    let ma = max(ma, tmp);
    let tmp = max(a2[m - 1].0 - b1[0].1, 0);
    let ma = max(ma, tmp);
    println!("{}", ma);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
