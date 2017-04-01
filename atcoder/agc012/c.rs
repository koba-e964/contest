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
    let mut n: i64 = get();
    let mut ans = Vec::new();
    let mut ans2 = Vec::new();
    n += 1;
    let mut ops = Vec::new(); 
    while n > 1 {
        if n % 2 == 1 {
            ops.push(1);
        }
        n /= 2;
        ops.push(0);
    }
    ops.reverse();
    let mut pos = 1;
    for v in ops {
        if v == 0 {
            // double
            ans.push(pos);
            ans2.push(pos);
        } else {
            ans.push(pos);
            ans2.insert(0, pos);
        }
        pos += 1;
    }
    ans.append(&mut ans2);
    println!("{}", ans.len());
    for i in 0 .. ans.len() {
        print!("{}{}", ans[i], if i == ans.len() - 1 { "\n" } else { " " });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
