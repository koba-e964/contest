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
    let k: usize = get();
    let s: i64 = get();
    let kk: i64 = k as i64;
    if kk <= s && s <= kk * (n - 1) {
        println!("YES");
        let mut ans = vec![1; k];
        let mut rem = s - kk;
        for i in 0 .. k {
            let v = min(rem, n - 2);
            ans[i] += v;
            rem -= v;
        }
        let mut cur = 1;
        for i in 0 .. k {
            let mut nxt = cur - ans[i];
            if nxt < 1 || nxt > n {
                nxt = cur + ans[i];
            }
            print!("{}{}", nxt, if i == k - 1 { "\n" } else { " " });
            cur = nxt;
        }
    } else {
        println!("NO");
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
