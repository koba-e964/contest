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

// https://atcoder.jp/contests/abc320/tasks/abc320_f
// DP の遷移が難しい。往路の場合は min(H, k - dist + f) だが袋の場合は min(H, k + f) - dist となる。
fn main() {
    input! {
        n: usize, h: usize,
        x: [usize; n],
        pf: [(i64, usize); n - 1],
    }
    let mut pf = pf;
    pf.push((0, 0));
    const INF: i64 = 1 << 60;
    let mut dp = vec![vec![INF; h + 1]; h + 1];
    for x in &mut dp[h] {
        *x = 0;
    }
    for i in 0..n {
        let mut ep = vec![vec![INF; h + 1]; h + 1];
        let dist = x[i] - if i == 0 { 0 } else { x[i - 1] };
        let (p, f) = pf[i];
        for j in dist..h + 1 {
            let nj0 = min(h, j - dist + f);
            let nj1 = j - dist;
            for k in 0..h + 1 {
                if min(h, k + f) >= dist {
                    let nk1 = min(h, k + f) - dist;
                    ep[nj1][k] = min(ep[nj1][k], dp[j][nk1] + p);
                }
                if k >= dist {
                    let nk0 = k - dist;
                    ep[nj0][k] = min(ep[nj0][k], dp[j][nk0] + p);
                    ep[nj1][k] = min(ep[nj1][k], dp[j][nk0]);
                }
            }
        }
        dp = ep;
    }
    let mut mi = INF;
    for i in 0..h + 1 {
        mi = min(mi, dp[i][i]);
    }
    println!("{}", if mi >= INF { -1 } else { mi });
}
