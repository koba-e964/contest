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
    let n: usize = get();
    let mut a = vec![0; n];
    for i in 0..(n + 4) / 5 {
        let lo = 5 * i;
        let hi = min(n, 5 * i + 5);
        let mut s = vec![0; n];
        let mut cur = 1;
        for i in lo..hi {
            s[i] = cur;
            cur *= 5;
        }
        print!("?");
        for i in 0..n {
            print!(" {}", s[i]);
        }
        println!();
        let mut ans: i64 = get();
        for i in lo..hi {
            ans -= 8;
            a[i] = (ans % 5) + 8;
            ans -= a[i] - 8;
            ans /= 5;
        }
    }
    print!("!");
    for i in 0..n {
        print!(" {}", a[i] % 2);
    }
    println!();
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
