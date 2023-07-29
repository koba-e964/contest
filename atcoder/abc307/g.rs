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

fn quo(x: i64, y: i64) -> i64 {
    assert!(y > 0);
    let q = x / y;
    let r = x - q * y;
    if r < 0 {
        q - 1
    } else {
        q
    }
}

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let asum: i64 = a.iter().sum();
    let q = quo(asum, n as i64);
    let r = (asum - q * n as i64) as usize;
    let mut a = a;
    for v in &mut a {
        *v -= q;
    }
    for i in 1..n {
        a[i] += a[i - 1];
    }
    const INF: i64 = 1 << 55;
    let mut dp = vec![INF; r + 1];
    dp[0] = 0;
    for i in 0..n {
        let mut ep = vec![INF; r + 1];
        for j in 0..r + 1 {
            ep[j] = (a[i] - j as i64).abs() + dp[j];
            if j > 0 {
                ep[j] = min(ep[j], (a[i] - j as i64).abs() + dp[j - 1]);
            }
        }
        dp = ep;
    }
    println!("{}", dp[r]);
}
