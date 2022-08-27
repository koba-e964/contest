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
        n: usize, k: usize,
        a: [i64; n],
    }
    let mut dp = vec![vec![0; n]; n];
    let mut acc = vec![vec![0; n]; n];
    let mut acc2 = vec![vec![0; n + 1]; n];
    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            let mut me = a[i] + a[j];
            if j >= k {
                me = max(me, acc[j - k + 1][i] + a[j]);
                me = max(me, acc2[i][j - k + 1] + a[j]);
            }
            dp[i][j] = me;
            acc[i + 1][j] = max(acc[i][j], me);
            acc2[i][j + 1] = max(acc2[i][j], me);
            ans = max(ans, me);
        }
    }
    println!("{}", ans);
}
