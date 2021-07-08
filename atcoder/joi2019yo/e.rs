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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Tags: range-max, segment-tree-less, interval-scheduling
fn main() {
    input! {
        n: usize, m: usize,
        a: [i64; n],
        lr: [(usize1, usize); m],
    }
    let mut mi = vec![0; n + 1];
    for i in 0..n {
        mi[i + 1] = i;
    }
    for &(l, r) in &lr {
        mi[r] = min(mi[r], l);
    }
    for i in (0..n).rev() {
        mi[i] = min(mi[i], mi[i + 1]);
    }
    let mut dp = vec![0; n + 1];
    for i in 1..n + 1 {
        dp[i] = max(dp[i - 1], dp[mi[i]] + a[i - 1]);
    }
    println!("{}", dp[n]);
}
