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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        n: usize,
        c: [[i64; n]; 1 << n],
    }
    let mut dp = vec![];
    for i in 0..1 << (n - 1) {
        let mut tmp = vec![0; n];
        for j in 0..n {
            tmp[j] = max(c[2 * i][j], c[2 * i + 1][j]);
        }
        dp.push(tmp);
    }
    for lv in 0..n - 1 {
        let mut ep = vec![];
        for j in 0..dp.len() / 2 {
            let mut tmp = vec![0; n];
            for k in lv + 1..n {
                tmp[k] = max(dp[2 * j][lv] + dp[2 * j + 1][k], dp[2 * j][k] + dp[2 * j + 1][lv]);
            }
            ep.push(tmp);
        }
        dp = ep;
    }
    println!("{}", dp[0][n - 1]);
}
