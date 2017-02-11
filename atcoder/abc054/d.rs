#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::*;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
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
            let ret = std::string::String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}
fn parse<T: std::str::FromStr>(s: &str) -> T { s.parse::<T>().ok().unwrap() }

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { parse(&get_word()) }

fn main() {
    let n = get();
    let ma: i32 = get();
    let mb: i32 = get();
    let mut tbl = Vec::new();
    for _ in 0 .. n {
        let a: i32 = get();
        let b: i32 = get();
        let c: usize = get();
        tbl.push((a * mb - b * ma, c))
    }
    const W: usize = 100000;
    const BIAS: usize = 50000;
    const INF: usize = 12345678;
    let mut dp = vec![vec![INF; W]; n + 1];
    for i in 0 .. n {
        let (delta, c) = tbl[i];
        for j in 0 .. W {
            if j as i32 + delta >= 0 && W as i32 > j as i32 + delta {
                let nj = (j as i32 + delta) as usize;
                dp[i + 1][nj] = min(dp[i + 1][nj], dp[i][j] + c);
                if j == BIAS {
                    dp[i + 1][nj] = min(dp[i + 1][nj], c);
                }
            }
            dp[i + 1][j] = min(dp[i + 1][j], dp[i][j]);
        }
    }
    if dp[n][BIAS] >= INF {
        println!("-1");
    } else {
        println!("{}", dp[n][BIAS]);
    }
}
