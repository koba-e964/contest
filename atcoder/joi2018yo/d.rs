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
    input! {
        n: usize,
        l: [i64; n],
    }
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + l[i];
    }
    const INF: i64 = 1 << 50;
    let mut mi = INF;
    for i in 0..n {
        for j in i + 1..n + 1 {
            if (i, j) == (0, n) {
                continue;
            }
            let d = acc[j] - acc[i];
            let mut dp = vec![0; n + 1];
            for k in 1..n + 1 {
                let mut me = INF;
                for l in if k == n { 1 } else { 0 }..k {
                    if acc[k] - acc[l] < d {
                        continue;
                    }
                    me.chmin(max(dp[l], acc[k] - acc[l]));
                }
                dp[k] = me;
            }
            mi.chmin(dp[n] - d);
        }
    }
    println!("{}", mi);
}
