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
        h: usize, w: usize,
        c: [chars; h],
    }
    const INF: i64 = 1 << 50;
    let mut ans = 1;
    let mut dp = vec![vec![-INF; w]; h];
    dp[0][0] = 1;
    for i in 0..h {
        for j in 0..w {
            if i + j == 0 { continue; }
            if c[i][j] == '#' {
                continue;
            }
            let mut ma = -INF;
            if i > 0 {
                ma = max(ma, dp[i - 1][j] + 1);
            }
            if j > 0 {
                ma = max(ma, dp[i][j - 1] + 1);
            }
            dp[i][j] = ma;
            ans = max(ans, ma);
        }
    }
    println!("{}", ans);
}
