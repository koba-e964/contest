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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// https://yukicoder.me/problems/no/2285 (3.5)
// Grundy 数 dp[n] を計算すれば良さそう。
// 実験の結果 n <= 10^5 での dp[n] の max は 9 (n = 88) だった。
// dp[n] = 9 なのは実験によると n = 88 + 34k (k >= 0) のときだった。
// さらに実験で n >= 89 のとき dp[n] == dp[n - 34] が分かった。
// なので n <= 88 の前計算ができていればよい。
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        t: usize,
        ab: [(usize, usize); t],
    }
    let n = 89;
    let mut dp = vec![0; n];
    for i in 4..n {
        let mut seen = HashSet::new();
        for j in 2..i - 1 {
            seen.insert(dp[j] ^ dp[i - j]);
        }
        let mut mex = 0;
        while seen.contains(&mex) { mex += 1; }
        dp[i] = mex;
    }
    fn red(a: usize) -> usize {
        if a <= 88 {
            a
        } else {
            (a - 89) % 34 + 89 - 34
        }
    }
    for (a, b) in ab {
        puts!("{}\n", if dp[red(a)] != dp[red(b)] { "First" } else { "Second" });
    }
}
