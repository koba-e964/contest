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

fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn mo(a: i64) -> i64 {
    (a % 606 + 606) % 606
}

fn det3(mat: &[Vec<i64>]) -> i64 {
    let mut ret = 0;
    for i in 0..3 {
        let mut prod = 1;
        for j in 0..3 {
            prod *= mat[j][(i + j) % 3];
        }
        ret += prod;
    }
    for i in 0..3 {
        let mut prod = 1;
        for j in 0..3 {
            prod *= mat[j][(i + 2 * j) % 3];
        }
        ret -= prod;
    }
    ret
}

// https://yukicoder.me/problems/no/2727 (3.5)
// 操作において j = k であれば体積が A[i]+1 倍になり、そうでなければ体積はそのままである。
// そのため実質的には、各プレイヤーが体積を A[i]+1 倍するかどうか、というゲームである。
// 6V mod 606 は 606 通りしかないので、これをキーにして DP をすればよい。
// Tags: dp
fn main() {
    let t: i32 = get();
    for _ in 0..t {
        let n: usize = get();
        let k: usize = get();
        let mut p = vec![vec![0i64; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                p[i][j] = get();
            }
        }
        let a: Vec<usize> = (0..n).map(|_| get()).collect();
        let s: Vec<_> = get_word().chars().collect();
        let mut dp = vec![[0; 606]; n + 1];
        for i in 0..k {
            dp[n][6 * i] = -1;
        }
        for i in k..101 {
            dp[n][6 * i] = 1;
        }
        for i in (0..n).rev() {
            if s[i] == 'K' {
                for j in 0..606 {
                    let mut ma = dp[i + 1][j];
                    ma = max(ma, dp[i + 1][j * (a[i] + 1) % 606]);
                    dp[i][j] = ma;
                }
            } else {
                for j in 0..606 {
                    let mut mi = dp[i + 1][j];
                    mi = min(mi, dp[i + 1][j * (a[i] + 1) % 606]);
                    dp[i][j] = mi;
                }
            }
        }
        let vraw = det3(&p).abs();
        if vraw == 0 {
            println!("D");
            continue;
        }
        let v = mo(vraw) as usize;
        println!("{}", if dp[0][v] == 1 { "K" } else if dp[0][v] == 0 { "D" } else { "P" });
    }
}
