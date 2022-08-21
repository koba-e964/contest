use std::collections::*;
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
        n: usize, m: usize,
        a: [(i64, i64); 3],
        xy: [(i64, i64); m],
    }
    let xy = xy.into_iter().collect::<HashSet<_>>();
    let mut dp = vec![vec![vec![0; n + 1]; n + 1]; n + 1];
    dp[0][0][0] = 1;
    const MOD: i64 = 998_244_353;
    for i in 0..n + 1 {
        for j in 0..n + 1 {
            for k in 0..n + 1 {
                if i + j + k == 0 { continue; }
                let mut x = 0;
                let mut y = 0;
                for l in 0..3 {
                    x += a[l].0 * [i, j, k][l] as i64;
                    y += a[l].1 * [i, j, k][l] as i64;
                }
                if xy.contains(&(x, y)) { continue; }
                let mut me = 0;
                if i > 0 {
                    me += dp[i - 1][j][k];
                }
                if j > 0 {
                    me += dp[i][j - 1][k];
                }
                if k > 0 {
                    me += dp[i][j][k - 1];
                }
                dp[i][j][k] = me % MOD;
            }
        }
    }
    let mut tot = 0;
    for i in 0..n + 1 {
        for j in 0..n + 1 - i {
            tot += dp[i][j][n - i - j];
        }
    }
    println!("{}", tot % MOD);
}
