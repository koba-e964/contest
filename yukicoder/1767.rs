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
        n: usize, m: usize,
        a: [i64; n],
        b: [i64; m],
    }
    const INF: i64 = 1 << 50;
    let mut pos = 0;
    let mut ans = 0;
    for i in 0..n + 1 {
        let lo = if i == 0 { -INF } else { a[i - 1] };
        let hi = if i == n { INF } else { a[i] };
        let mut dp = vec![lo];
        while pos < m && b[pos] < hi {
            dp.push(b[pos]);
            pos += 1;
        }
        dp.push(hi);
        let mut ma = 0;
        for i in 0..dp.len() - 1 {
            ma = max(ma, dp[i + 1] - dp[i]);
        }
        ans += hi - lo - ma;
    }
    println!("{}", ans);
}
