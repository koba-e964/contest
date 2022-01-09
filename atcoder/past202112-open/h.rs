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

fn main() {
    input! {
        s: chars,
        t: chars,
    }
    let n = s.len();
    let m = t.len();
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 0..n + 1 {
        for j in 0..m + 1 {
            if i + j == 0 { continue; }
            let mut me = 0;
            if i > 0 {
                me = max(me, dp[i - 1][j]);
            }
            if j > 0 {
                me = max(me, dp[i][j - 1]);
            }
            if i > 0 && j > 0 {
                me = max(me, dp[i - 1][j - 1] + if s[i - 1] == t[j - 1] {
                    0
                } else {
                    1
                });
            }
            dp[i][j] = me;
        }
    }
    println!("{}", dp[n][m]);
}
