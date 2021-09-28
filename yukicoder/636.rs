use std::cmp::*;

fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

const INF: i32 = 1 << 28;

fn f(c: i32) -> i32 {
    if c < 0 {
        INF
    } else {
        c / 10 + (c % 10) / 5 + c % 5
    }
}

// Tags: digital-dp
fn main() {
    let s: Vec<_> = getline().trim().bytes().collect();
    let n = s.len();
    let mut dp = vec![[INF; 3]; n + 1];
    dp[n][0] = 0;
    for i in (0..n).rev() {
        let c = (s[i] - b'0') as i32;
        for b in 0..2 {
            let c = c + b as i32;
            for j in 0..11 {
                dp[i][0] = min(dp[i][0], dp[i + 1][b] + f(j - c) + f(j));
                dp[i][1] = min(dp[i][1], dp[i + 1][b] + f(j) + f(j - c + 10));
            }
        }
    }
    println!("{}", dp[0][0]);
}
