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
        n: usize, m: usize,
        x: [i64; n],
        cy: [(usize, i64); m],
    }
    let mut bonus = vec![0; n + 1];
    for (c, y) in cy {
        bonus[c] = y;
    }
    const INF: i64 = 1 << 50;
    let mut dp = vec![-INF; n + 1];
    dp[0] = 0;
    for i in 0..n {
        let mut ep = vec![-INF; n + 1];
        for j in 0..n + 1 {
            ep[0] = max(ep[0], dp[j]);
            if j < n {
                ep[j + 1] = max(ep[j + 1], dp[j] + x[i] + bonus[j + 1]);
            }
        }
        dp = ep;
    }
    println!("{}", dp.iter().max().unwrap());
}
