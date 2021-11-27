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
        a: [i64; n],
        b: [i64; n],
    }
    const INF: i64 = 1 << 50;
    let mut ans = INF;
    for c0 in 0..2 {
        let mut dp = vec![[INF; 2]; n + 1];
        dp[0][c0] = 0;
        for i in 1..n + 1 {
            let a = a[i % n];
            let b = b[i - 1];
            let pre = dp[i - 1];
            let mut me = [INF; 2];
            me[0] = a + min(pre[0] + b, pre[1]);
            me[1] = min(pre[0], pre[1] + b);
            dp[i] = me;
        }
        ans = min(ans, dp[n][c0]);
    }
    println!("{}", ans);
}
