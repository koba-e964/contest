#[allow(unused_imports)]
use std::cmp::*;
use std::io::*;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}
#[allow(dead_code)]
fn getword() -> String {
    let mut stdin = std::io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.is_err() ||u8b[0] <= ' ' as u8 {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = std::string::String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}
#[allow(dead_code)]
fn parse<T : std::str::FromStr>(s : &str) -> T {
     return s.parse::<T>().ok().unwrap();
}

const MINF: i32 = -300 * 3000 - 10;

fn main() {
    let n = parse(&getword());
    let m: usize = parse(&getword());
    let mut w = vec![0; n];
    for i in 0 .. n {
        w[i] = parse(&getword());
    }
    let mut dp: Vec<Vec<[[i32; 2]; 2]>> = vec![vec![[[MINF; 2]; 2]; n + 1]; n + 1];

    dp[0][0][0][0] = 0;
    dp[1][0][0][0] = 0;
    dp[1][1][1][1] = 0;
    for i in 2 .. n {
        dp[i][0][0][0] = 0;
        for j in 1 .. (n + 1) {
            for s in 0 .. 2 {
                for t in 0 .. 2 {
                    dp[i][j][0][t] = max(dp[i][j][0][t], dp[i - 1][j][s][t]);
                    if dp[i - 1][j - 1][s][t] > MINF {
                        dp[i][j][1][t] = max(dp[i][j][1][t], dp[i - 1][j - 1][s][t] + (s as i32) * w[i - 2]);
                    }
                }
            }
        }
    }
    dp[n][0][0][0] = 0;
    for j in 1 .. (n + 1) {
        for s in 0 .. 2 {
            for t in 0 .. 2 {
                dp[n][j][0][t] = max(dp[n][j][0][t], dp[n - 1][j][s][t]);
                if dp[n - 1][j - 1][s][t] > MINF {
                    dp[n][j][1][t] = max(dp[n][j][1][t], dp[n - 1][j - 1][s][t] + (s as i32) * w[n - 2] + (t as i32) * w[n - 1]);
                }
            }
        }
    }
    let mut ma = MINF;
    for s in 0 .. 2 {
        for t in 0 .. 2 {
            ma = max(ma, dp[n][m][s][t]);
        }
    }
    println!("{}", ma);
}
