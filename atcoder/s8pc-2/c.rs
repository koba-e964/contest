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

const MOD: i64 = 1_000_000_007;

fn solve() {
    let n: Vec<usize> = get_word()
        .bytes()
        .map(|x| (x - b'0') as usize)
        .collect();
    let d: usize = get();
    let w = n.len();
    let mut dp = vec![vec![0i64; d + 1]; w + 1];
    dp[0][0] = 1;
    for i in 1 .. w + 1 {
        for j in 1 .. i + 1 {
            let mut seg = 0;
            let mut ok = true;
            for k in i - j .. i {
                seg *= 10;
                seg += n[k];
                if seg > d {
                    ok = false;
                    break;
                }
            }
            if !ok {
                break;
            }
            for k in seg .. d + 1 {
                let tmp = dp[i - j][k - seg];
                dp[i][k] += tmp;
                dp[i][k] %= MOD;
            }
        }
    }
    let mut sum: i64 = 0;
    for i in 0 .. d + 1 {
        sum += dp[w][i];
        sum %= MOD;
    }
    println!("{}", sum);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
