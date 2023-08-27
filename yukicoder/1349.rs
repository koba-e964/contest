#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

// https://yukicoder.me/problems/no/1349 (3.5, 解説を見た)
// N 個の要素を大きさ 10 程度のブロックに分割して、ブロックごとに区間和を持つと N^2/200 エントリで済む。
// -> 解説を見た。dp[i][j] := min {x | [i, x] で j を作れる} とし、
// dp[i][k * a[i]] <- dp[i + 1][k] という DP をする。
// -> a[i] を使わない遷移 (dp[i][k] <- dp[i + 1][k]) を忘れて WA。
// The author read the editorial before implementing this.
// Tags: idea, queries
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, q: usize, p: usize,
        a: [usize; n],
        lrk: [(usize1, usize, usize); q],
    }
    let mut dp = vec![vec![n; p]; n + 1];
    for i in (0..n).rev() {
        dp[i][a[i]] = i;
        for j in 0..p {
            let tmp = dp[i + 1][j];
            dp[i][(j * a[i]) % p].chmin(tmp);
            dp[i][j].chmin(tmp);
        }
    }
    for (l, r, k) in lrk {
        puts!("{}\n", if dp[l][k] < r { "Yes" } else { "No" });
    }
}
