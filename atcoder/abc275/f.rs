use std::cmp::*;
use std::io::{Write, BufWriter};
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
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize,
        a: [usize; n],
    }
    const INF: i32 = 1 << 28;
    let mut dp = vec![[INF; 2]; m + 1];
    dp[0][1] = 0;
    for i in 0..n {
        let mut ep = vec![[INF; 2]; m + 1];
        for j in 0..m + 1 {
            for b in 0..2 {
                ep[j][0].chmin(dp[j][b] + b as i32);
            }
            if j + a[i] <= m {
                for b in 0..2 {
                    ep[j + a[i]][1].chmin(dp[j][b]);
                }
            }
        }
        dp = ep;
    }
    for x in 1..m + 1 {
        let &ans = dp[x].iter().min().unwrap();
        puts!("{}\n", if ans >= INF { -1 } else { ans });
    }
}

