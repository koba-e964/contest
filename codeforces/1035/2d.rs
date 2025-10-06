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

fn addm(a: i64, b: i64, m: i64) -> i64 {
    let mut r = a + b;
    if r >= m {
        r -= m;
    }
    r
}

// この問題では普通の箱根駅伝 DP とは違い、右側の頂点は 「自分以上とマッチする」か「マッチしない」の 2 択になる。
// それに応じて 2nd index も「右側のマッチされていない頂点が何個か」にする必要がある。
// Tags: counting-permutations, hakone-ekiden-dp
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        t: usize,
        nm: [(usize, i64); t],
    }
    for (n, m) in nm {
        let mut dp = vec![vec![0; n + 2]; n + 1];
        dp[0][0] = 1;
        for i in 0..n {
            for j in 0..i + 1 {
                let val = dp[i][j];
                // (=, =)
                dp[i + 1][j] = addm(dp[i + 1][j], val * (i + 1) as i64 % m, m);
                // (none, none) or (none, >)
                // (none, none)
                dp[i + 1][j] = addm(dp[i + 1][j], val, m);
                // (none, >)
                dp[i + 1][j + 1] = addm(dp[i + 1][j + 1], val * (i + 1) as i64 % m, m);
                // (>, none) or (>, >)
                if j > 0 {
                    // (>, none)
                    dp[i + 1][j - 1] = addm(dp[i + 1][j - 1], val * j as i64 % m, m);
                    // (>, >)
                    dp[i + 1][j] = addm(dp[i + 1][j], val * (i + 1) as i64 % m * j as i64 % m, m);
                }
            }
        }
        puts!("{}\n", dp[n][0]);
    }
}
