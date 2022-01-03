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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        gx: usize, gy: usize, n: usize, f: i64,
        xyc: [(usize, usize, i64); n],
    }
    const INF: i64 = 1 << 50;
    let mut dp = vec![vec![vec![INF; n + 1]; gy + 1]; gx + 1];
    dp[0][0] = vec![0; n + 1];
    for i in 0..gx + 1 {
        for j in 0..gy + 1 {
            if i + j == 0 { continue; }
            for k in 0..n + 1 {
                let mut me = INF;
                if i > 0 {
                    me = min(me, dp[i - 1][j][k] + f);
                }
                if j > 0 {
                    me = min(me, dp[i][j - 1][k] + f);
                }
                if k > 0 {
                    let (x, y, c) = xyc[k - 1];
                    if i >= x && j >= y {
                        me = min(me, dp[i - x][j - y][k - 1] + c);
                    }
                    me = min(me, dp[i][j][k - 1]);
                }
                dp[i][j][k] = me;
            }
        }
    }
    println!("{}", dp[gx][gy][n]);
}
