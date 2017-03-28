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

fn check(v: i32, a: &[i32]) -> Option<i64> {
    let mut tot = 0;
    for &e in a.iter() {
        let e = e as i64;
        let v = v as i64;
        if e >= v * v {
            // Find (x, y) s.t. x * v + y * (v + 1) = e
            let mut r = v + 1 - e % (v + 1);
            if r >= v + 1 {
                r = 0;
            }
            tot += r + (e - r * v) / (v + 1);
            continue;
        }
        tot += e / v;
        let q = e / v;
        let r = e % v;
        if q < r {
            return None;
        }
    }
    Some(tot)
}

fn solve() {
    let n = get();
    let mut a: Vec<i32> = (0 .. n).map(|_| get()).collect();
    a.sort();
    let cand_set;
    {
        let v = a[0];
        let mut cur_cand = HashSet::with_capacity(80000);
        let mut i = 1;
        while i * i <= v {
            cur_cand.insert(v / i);
            if v % i == 0 && v > i {
                cur_cand.insert(v / i - 1);
            }
            cur_cand.insert(i);
            i += 1;
        }
        cand_set = cur_cand;
    }
    let mut cand: Vec<_> = cand_set.into_iter().collect();
    cand.sort();
    cand.reverse();
    for v in cand {
        if let Some(tot) = check(v, &a) {
            println!("{}", tot);
            return;
        }
    }
    panic!();
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
