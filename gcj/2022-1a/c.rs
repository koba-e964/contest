use std::cmp::*;
use std::io::Read;

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

fn main() {
    let t: usize = get();
    for i in 0..t {
        let e: usize = get();
        let w: usize = get();
        let mut x = vec![vec![0; w]; e];
        for j in 0..e {
            for k in 0..w {
                x[j][k] = get();
            }
        }
        x.insert(0, vec![0; w]);
        x.push(vec![0; w]);
        println!("Case #{}: {}", i + 1, calc(x));
    }
}

fn calc(x: Vec<Vec<i32>>) -> i32 {
    let n = x.len();
    let m = x[0].len();
    let mut mi = vec![vec![vec![0; m]; n + 1]; n];
    for i in 0..n {
        for j in 0..m {
            mi[i][i + 1][j] = x[i][j];
        }
    }
    for s in 2..n + 1 {
        for i in 0..n - s + 1 {
            let j = i + s;
            for k in 0..m {
                mi[i][j][k] = min(mi[i][j - 1][k], x[j - 1][k]);
            }
        }
    }
    let mut dp = vec![vec![0; n + 1]; n];
    // n^3 m / 6
    for s in 2..n + 1 {
        for i in 0..n - s + 1 {
            let j = i + s;
            let mut ans = 1 << 30;
            for k in i + 1..j {
                let mut sum = 0;
                for l in 0..m {
                    sum += 2 * (mi[i][k][l] - mi[k][j][l]).abs();
                }
                ans = min(ans, sum + dp[i][k] + dp[k][j]);
            }
            dp[i][j] = ans;
        }
    }
    dp[0][n]
}
