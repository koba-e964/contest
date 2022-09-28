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
        n: usize,
        ab: [(i64, i64); n],
    }
    let mut ab = ab;
    ab.sort_by_key(|&(_, b)| b);
    const INF: i64 = 1 << 50;
    let mut dp = vec![vec![INF; n]; 3];
    dp[0][0] = ab[0].0;
    for i in 1..n {
        dp[0][i] = min(dp[0][i - 1], ab[i].0);
    }
    for i in 1..3 {
        for j in 1..n {
            let (a, b) = ab[j];
            dp[i][j] = min(dp[i][j - 1], dp[i - 1][j - 1] + a + b);
        }
    }
    println!("{}", dp[2][n - 1]);
}
