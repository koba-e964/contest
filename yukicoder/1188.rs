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

fn lis(a: &[i64]) -> Vec<usize> {
    let n = a.len();
    const INF: i64 = 1 << 60;
    let mut dp = vec![INF; n + 1];
    let mut ans = vec![0; n];
    dp[0] = -INF;
    for i in 0..n {
        let mut pass = 0;
        let mut fail = n;
        while fail - pass > 1 {
            let mid = (fail + pass) / 2;
            if dp[mid] < a[i] {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        ans[i] = pass + 1;
        dp[pass + 1] = min(dp[pass + 1], a[i]);
    }
    ans
}

fn calc(a: &[i64]) -> usize {
    let lis1 = lis(a);
    let mut a = a.to_vec();
    a.reverse();
    let mut lis2 = lis(&a);
    lis2.reverse();
    let mut ans = 0;
    for i in 0..a.len() {
        ans = max(ans, min(lis1[i], lis2[i]) - 1);
    }
    ans
}

// Tags: longest-increasing-subsequences    
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut a = a;
    let b = calc(&a);
    for i in 0..n {
        a[i] = -a[i];
    }
    let b = max(b, calc(&a));
    println!("{}", b);
}
