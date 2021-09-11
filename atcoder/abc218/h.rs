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

fn calc(n: usize, a: &[i64], x: i64) -> (i64, usize) {
    let mut dp = vec![(0, 0); n + 1];
    for i in 0..n {
        let sc = if i == 0 {
            a[i]
        } else if i == n - 1 {
            a[i - 1]
        } else {
            a[i] + a[i - 1]
        };
        let mut ma = dp[i];
        let val = if sc >= x { (sc - x, 1) } else { (0, 0) };
        let pre = if i == 0 { dp[0] } else { dp[i - 1] };
        let tmp = (pre.0 + val.0, pre.1 + val.1);
        ma = max(ma, tmp);
        dp[i + 1] = ma;
    }
    dp[n]
}

fn main() {
    input! {
        n: usize, r: usize,
        a: [i64; n - 1],
    }
    let r = min(r, n - r);
    let mut pass = 0;
    let mut fail = 1 << 40;
    while fail - pass > 1 {
        let mid = (pass + fail) / 2;
        let sub = calc(n, &a, mid);
        if sub.1 >= r {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    eprintln!("pass = {}", pass);
    let sub = calc(n, &a, pass);
    eprintln!("sub = {:?}", sub);
    println!("{}", sub.0 + pass * r as i64);
}
