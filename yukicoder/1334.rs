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

const MOD: i64 = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let inf = 400_001;
    let mut one = true;
    let mut l = 0;
    let mut v = vec![];
    let mut c = 1;
    let mut d = 1;
    for i in 0..n {
        if one != (a[i] == 1) {
            v.push((l, c, d));
            one = a[i] == 1;
            l = 1;
            c = a[i];
            d = a[i];
        } else {
            c = min(inf, c * a[i]);
            d = d * a[i] % MOD;
            l += 1;
        }
    }
    v.push((l, c, d));
    if v.len() % 2 == 0 {
        v.push((0, 1, 1));
    }
    let m = v.len() / 2;
    if m == 0 {
        println!("{}", v[0].0);
        return;
    }
    c = 1;
    d = 1;
    for i in 0..m {
        let (_, cc, dd) = v[2 * i + 1];
        c = min(inf, c * cc);
        d = d * dd % MOD;
    }
    if c >= inf {
        println!("{}", (d + v[0].0 + v[2 * m].0) % MOD);
        return;
    }
    let mut dp = vec![0; m + 1];
    for i in 0..m {
        let mut ans = 0;
        for j in 0..i + 1 {
            let mut p = 1;
            for k in j..i + 1 {
                p *= v[2 * k + 1].1;
            }
            ans = max(ans, v[2 * j].0 + p + dp[j]);
        }
        dp[i + 1] = ans;
    }
    println!("{}", dp[m] + v[2 * m].0);
}
