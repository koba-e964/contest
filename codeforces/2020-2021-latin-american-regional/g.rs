use std::cmp::*;
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes.by_ref().map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input!(n: usize);
    let mut dp = vec![vec![vec![0.0; n + 1]; n + 1]; n + 1];
    for i in 0..n + 1 {
        for j in 0..n + 1 {
            if i + j == 0 {
                continue;
            }
            let p = i as f64 / (i + j) as f64;
            let q = j as f64 / (i + j) as f64;
            for k in 0..n + 1 {
                if i > 0 {
                    dp[i][j][k] += dp[i - 1][j][max(k, 1) - 1] * p;
                    if k == 0 {
                        dp[i][j][k] += i as f64 * p;
                    }
                }
                if j > 0 && k < n {
                    dp[i][j][k] += dp[i][j - 1][k + 1] * q;
                }
            }
        }
    }
    println!("{:.6}", dp[n][n][0]);
}
