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
    let h: usize = get();
    let n: usize = get();
    let p: usize = get::<usize>() - 1;
    let m: usize = get();
    let k: usize = get();
    let mut yoko = vec![None; h];
    for _ in 0 .. m {
        let a: usize = get();
        let b: usize = get();
        yoko[a - 1] = Some(b - 1);
    }
    let mut dp = vec![vec![vec![0.0; n]; k + 1]; h + 1];
    dp[h][0][p] = 1.0;
    let mut rem = h - m;
    for i in (0 .. h).rev() {
        if let Some(pos) = yoko[i] {
            dp[i] = dp[i + 1].clone();
            for j in 0 .. k + 1 {
                dp[i][j].swap(pos, pos + 1);
            }
            continue;
        }
        for j in 0 .. k + 1 {
            for l in 0 .. n {
                let coef = (rem as f64 - (k - j) as f64) / rem as f64;
                dp[i][j][l] += coef * dp[i + 1][j][l];
                if j > 0 {
                    let coef = (k - j + 1) as f64 / rem as f64 / (n - 1) as f64;
                    if n >= 3 {
                        dp[i][j][l] += (n - 3) as f64 * coef * dp[i + 1][j - 1][l];
                        dp[i][j][l] += coef * dp[i + 1][j - 1]
                            [if l == 0 { 0 } else { l - 1 }];
                        dp[i][j][l] += coef * dp[i + 1][j - 1]
                            [if l == n - 1 { n - 1 } else { l + 1 }];
                    } else {
                        dp[i][j][l] += coef * dp[i + 1][j - 1][1 - l];
                    }
                }
            }
        }
        rem -= 1;
    }
    println!("{:.15}", dp[0][k].iter()
             .max_by(|&x, &y| x.partial_cmp(y).unwrap()).unwrap());
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
