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

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn main() {
    const INF: f64 = 1.0e18;
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut pass = 0.0;
    let mut fail = 1.1e9;
    for _ in 0..60 {
        let mid = (pass + fail) / 2.0;
        let mut b = vec![0.0; n];
        for i in 0..n {
            b[i] = a[i] as f64 - mid;
        }
        let mut dp = vec![[-INF; 2]; n + 1];
        dp[0] = [0.0; 2];
        for i in 0..n {
            let mut me = [-INF; 2];
            me[0].chmax(dp[i][1]);
            me[1].chmax(dp[i][1] + b[i]);
            me[1].chmax(dp[i][0] + b[i]);
            dp[i + 1] = me;
        }
        if dp[n][0].max(dp[n][1]) >= 0.0 {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    println!("{}", pass);
    let mut pass = 0;
    let mut fail = 1i64 << 30;
    let inf = 1i64 << 40;
    while fail - pass > 1 {
        let mid = (fail + pass) / 2;
        let mut dp = vec![[-inf; 2]; n + 1];
        dp[0] = [0; 2];
        for i in 0..n {
            let mut me = [-inf; 2];
            let this = if a[i] >= mid {
                1
            } else {
                -1
            };
            me[0].chmax(dp[i][1]);
            me[1].chmax(dp[i][1] + this);
            me[1].chmax(dp[i][0] + this);
            dp[i + 1] = me;
        }
        if max(dp[n][0], dp[n][1]) >= 1 {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    println!("{}", pass);
}
