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

fn calc(h: usize, w: usize, obst: &[(usize, usize)]) -> i64 {
    let mut dp = vec![vec![0; w + 1]; h + 1];
    dp[0][0] = 1;
    for &(x, y) in obst.iter() {
        dp[x][y] = -1;
    }
    for i in 0 .. h + 1 {
        for j in 0 .. w + 1 {
            if i == 0 && j == 0 { continue; }
            if dp[i][j] == -1 {
                dp[i][j] = 0;
                continue;
            }
            let mut tot = 0;
            if i > 0 {
                tot += dp[i - 1][j];
            }
            if j > 0 {
                tot += dp[i][j - 1];
            }
            dp[i][j] = tot;
        }
    }
    dp[h][w]
}

fn solve() {
    let h = get();
    let w = get();
    let k = get();
    let p = get();
    let mut x = vec![0; k];
    let mut y = vec![0; k];
    let mut name = vec![String::new(); k];
    for i in 0 .. k {
        x[i] = get();
        y[i] = get();
        name[i] = get_word();
    }
    let mut ma: i64 = 0;
    let mut maxi = 0;
    for bits in 0 .. 1usize << k {
        if bits.count_ones() != p {
            continue;
        }
        let mut obst = Vec::new();
        for i in 0 .. k {
            if (bits & 1 << i) == 0 {
                obst.push((x[i], y[i]));
            }
        }
        let ans = calc(h, w, &obst);
        if ma < ans {
            ma = ans;
            maxi = bits;
        }
    }
    println!("{}", ma % 1_000_000_007);
    for i in 0 .. k {
        if (maxi & 1 << i) != 0 {
            println!("{}", name[i]);
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
