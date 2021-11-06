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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Tags: interval-dp
fn main() {
    input! {
        n: usize,
        s: chars,
    }
    let pon = ['p', 'o', 'n'];
    const INF: i32 = 1 << 28;
    let mut dp = vec![vec![[-INF; 3]; n + 1]; n];
    for i in 0..n {
        for j in i..n + 1 {
            dp[i][j][0] = 0;
        }
    }
    for t in 3..n + 1 {
        for i in 0..n - t + 1 {
            let j = i + t;
            for b in 0..3 {
                let mut ma = -INF;
                if b <= 1 {
                    for k in 0..3 {
                        if s[i..i + k] == pon[..k] && s[j + k - 3..j] == pon[k..]
                        {
                            ma = max(ma, 1 + dp[i + k][j + k - 3][0]);
                        }
                    }
                }
                for c in 0..b + 1 {
                    for k in i + 1..j {
                        ma = max(ma, dp[i][k][c] + dp[k][j][b - c]);
                    }
                }
                if s[i] == 'p' && s[j - 1] == 'n' && b <= 1 {
                    for k in i + 1..j - 1 {
                        if s[k] == 'o' && dp[i + 1][k][0] >= 0 && dp[k + 1][j - 1][0] >= 0 {
                            ma = max(ma, 1 + dp[i + 1][k][0] + dp[k + 1][j - 1][0]);
                        }
                    }
                }
                ma = max(ma, dp[i][j - 1][b]);
                ma = max(ma, dp[i + 1][j][b]);
                dp[i][j][b] = ma;
            }
        }
    }
    println!("{}", max(-1, dp[0][n][2] - 2));
}
