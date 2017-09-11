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
    let n: i64 = get();
    let q = get();
    let a: Vec<i64> = (0 .. q).map(|_| get()).collect();
    let mut cnt = 0;
    let mut occur = HashSet::new();
    for bits in 0 .. 1 << q {
        let mut prod = 1;
        let mut ok = true;
        for i in 0 .. q {
            if (bits & 1 << i) != 0 {
                prod *= a[i];
                if n < prod {
                    ok = false;
                    break;
                }
            }
        }
        if !ok || n < prod {
            continue;
        }
        if occur.contains(&prod) { continue; }
        occur.insert(prod);
        // trial division
        for &v in a.iter() {
            if prod % v == 0 {
                prod /= v;
            }
        }
        if prod == 1 {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
