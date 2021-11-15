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

// https://yukicoder.me/problems/no/595 (3)
// 区間に分けたい。
// 右から見たときと左から見た時の登る距離は累積和にできる。
// 更新式は dp[j] = min(dp[i] + acc_l[j - 1] - acc_l[i], dp[i] + acc_r[i] - acc_r[j - 1]) + p という形なので、左から dp[i] - acc_l[i] と dp[i] + acc_r[i] の累積 min を計算していけば良い。
// Tags: dp
fn main() {
    input! {
        n: usize, p: i64,
        h: [i64; n],
    }
    let mut acc_l = vec![0; n];
    for i in 0..n - 1 {
        acc_l[i + 1] = acc_l[i] + max(0, h[i + 1] - h[i]);
    }
    let mut acc_r = vec![0; n];
    for i in (0..n - 1).rev() {
        acc_r[i] = acc_r[i + 1] + max(0, h[i] - h[i + 1]);
    }
    let mut dp = vec![0; n + 1];
    let mut mim = 0;
    let mut mip = acc_r[0];
    for j in 1..n + 1 {
        dp[j] = min(acc_l[j - 1],
                    min(mim + acc_l[j - 1], mip - acc_r[j - 1]) + p);
        if j < n {
            mim = min(mim, dp[j] - acc_l[j]);
            mip = min(mip, dp[j] + acc_r[j]);
        }
    }
    println!("{}", dp[n]);
}
