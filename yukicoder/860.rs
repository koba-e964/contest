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

const INF: i64 = 1 << 50;

// https://yukicoder.me/problems/no/860 (3)
// sum (min C - D_i) の最小化。-> よく読むと列を複数の区間へと分割する問題だった。
// ある日に [i, j) の商品を買うとすると、C_? + D_? の和の他に余計にかかるコストは min_{i <= x < j} C[x] - D[i] である。
// 右から DP を行うと dp[i].chmin(dp[j] + range_min(C[i..j]) - D[i]) である。右から見て単調増加になるようにスタックに C[i] と、次の要素が C[j] だったとして range_min(dp[i + 1..j + 1]) を格納することにすると、スタックの中の C[i] + range_min(dp[i..j]) の最小値が常にわかっていれば計算ができる。
// Tags: stack, dp
fn main() {
    input! {
        n: usize,
        cd: [(i64, i64); n],
    }
    let s: i64 = cd.iter().map(|&(c, d)| c + d).sum();
    let mut dp = vec![0; n + 1];
    let mut st = vec![(cd[n - 1].0, 0, cd[n - 1].0)];
    for i in (0..n).rev() {
        let (c, d) = cd[i];
        if i + 1 < n {
            let mut ndp = dp[i + 1];
            while let Some((sc, sdp, accmi)) = st.pop() {
                if sc >= c {
                    ndp = min(ndp, sdp);
                    continue;
                }
                st.push((sc, sdp, accmi));
                break;
            }
            let accmi = if let Some(&v) = st.last() {
                v.2
            } else {
                INF
            };
            st.push((c, ndp, min(accmi, c + ndp)));
        }
        dp[i] = -d + st.last().unwrap().2;
    }
    println!("{}", s + dp[0]);
}
