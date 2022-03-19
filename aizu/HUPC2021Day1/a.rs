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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        m: usize, n: usize,
        s: [chars; m],
    }
    let mut ans = 1 << 30;
    let mut dp = vec![vec![0; n + 1]; m + 1];
    for i in 0..m {
        for j in 0..n {
            dp[i + 1][j + 1] = dp[i + 1][j] + dp[i][j + 1] - dp[i][j]
                + if s[i][j] == 'o' { 1 } else { 0 };
        }
    }
    for i in 1..m {
        let x = dp[i][n];
        let y = dp[m][n] - dp[i][n];
        ans = min(ans, x + ((m - i) * n) as i32 - y);
        ans = min(ans, (i * n) as i32 - x + y);
    }
    for i in 1..n {
        let x = dp[m][i];
        let y = dp[m][n] - dp[m][i];
        ans = min(ans, x + (m * (n - i)) as i32 - y);
        ans = min(ans, (m * i) as i32 - x + y);
    }
    println!("{}", ans);
}
