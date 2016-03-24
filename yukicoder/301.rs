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

const N: usize = 10000;

fn calc(dp: &[[f64; 2]], n: usize) -> f64 {
    if n >= N {
        return (n as f64) + 5.0 / 3.0;
    }
    return dp[n][0] / (1.0 - dp[n][1]);
}

/**
 * Reference: http://yukicoder.me/submissions/77166
 */ 
fn main() {
    let t: usize = parse(&getword());
    let mut dp = vec![[0.0_f64; 2]; N];
    for i in 1 .. N {
        for j in (i as i32) - 6 .. i as i32 {
            if j < 0 {
                dp[i][0] += 1.0 / 6.0;
                dp[i][1] += 1.0 / 6.0;
            } else {
                dp[i][0] += (dp[j as usize][0] + 1.0) / 6.0;
                dp[i][1] += dp[j as usize][1] / 6.0;
            }
        }
    }
    for _ in 0 .. t {
        let n = parse(&getword());
        println!("{}", calc(&dp, n));
    }
}
