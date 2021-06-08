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

// Tags: expectation, dice
fn main() {
    loop {
        let n: usize = get();
        if n == 0 {
            break;
        }
        let x: Vec<i32> = (0..n).map(|_| get()).collect();
        let mut lb = vec![0.0; n];
        let mut ro = vec![0.0; n];
        for i in (0..n - 1).rev() {
            if x[i] > 0 {
                lb[i] = lb[i + x[i] as usize];
                ro[i] = ro[i + x[i] as usize];
            } else if x[i] == -1 {
                ro[i] = 0.0;
                lb[i] = 1.0;
            } else {
                for y in 1..=6 {
                    ro[i] += ro[min(n - 1, i + y)];
                    lb[i] += lb[min(n - 1, i + y)];
                }
                ro[i] /= 6.0;
                lb[i] /= 6.0;
                ro[i] += 1.0;
            }
        }
        println!("{}", ro[0] / (1.0 - lb[0]));
    }
}
