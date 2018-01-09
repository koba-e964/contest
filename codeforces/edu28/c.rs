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
    let n = get();
    let a: Vec<i64> = (0 .. n).map(|_| get()).collect();
    let mut acc_l = vec![0; n + 1];
    for i in 0 .. n {
        acc_l[i + 1] = acc_l[i] + a[i];
    }
    let mut acc_r = vec![0; n + 1];
    for i in (0 .. n).rev() {
        acc_r[i] = acc_r[i + 1] + a[i];
    }
    let mut delim_l = vec![0; n + 1];
    let mut delim_lind = vec![0; n + 1];
    {
        let mut ma = 0;
        let mut maxi = 0;
        for i in 0 .. n {
            if ma < acc_l[i + 1] {
                ma = acc_l[i + 1];
                maxi = i + 1;
            }
            delim_l[i + 1] = 2 * ma - acc_l[i + 1];
            delim_lind[i + 1] = maxi;
        }
    }
    let mut delim_r = vec![0; n + 1];
    let mut delim_rind = vec![0; n + 1];
    {
        let mut mi = 0;
        let mut mini = n;
        for i in (0 .. n).rev() {
            if mi > acc_r[i] {
                mi = acc_r[i];
                mini = i;
            }
            delim_r[i] = - 2 * mi + acc_r[i];
            delim_rind[i] = mini;
        }
    }
    let mut ma = 0;
    let mut maxi = 0;
    for i in 0 .. n + 1 {
        if ma < delim_l[i] + delim_r[i] {
            ma = delim_l[i] + delim_r[i];
            maxi = i;
        }
    }
    let left = delim_lind[maxi];
    let right = delim_rind[maxi];
    println!("{} {} {}", left, maxi, right);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
