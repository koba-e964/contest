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

fn calc(s: &[char]) -> Vec<[usize; 26]> {
    let n = s.len();
    let mut dp = vec![[n; 26]; n + 1];
    for i in (0..n).rev() {
        dp[i] = dp[i + 1];
        let idx = (s[i] as u8 - b'a') as usize;
        dp[i][idx] = i;
    }
    dp
}

// Tags: lcs
fn main() {
    input! {
        s: chars,
        t: chars,
        k: usize,
    }
    let n = s.len();
    let m = t.len();
    let f = calc(&s);
    let g = calc(&t);
    let mut dp = vec![vec![vec![0; k + 1]; m + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            for l in 0..k + 1 {
                let mut me = if l == k { 0 } else { dp[i + 1][j + 1][l + 1] + 1 };
                for u in 0..26 {
                    if f[i][u] < n && g[j][u] < m {
                        me = max(me, dp[f[i][u] + 1][g[j][u] + 1][l] + 1);
                    }
                }
                dp[i][j][l] = me;
            }
        }
    }
    let mut ans = 0;
    for i in 0..n + 1 {
        for j in 0..m + 1 {
            ans = max(ans, dp[i][j][0]);
        }
    }
    println!("{}", ans);
}
