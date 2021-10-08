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
        n: usize, v: i64,
        c: [i64; n],
    }
    let mut d = vec![0; n];
    d[0] = c[0];
    for i in 1..n {
        d[i] = d[i - 1] + c[i];
    }
    if v < n as i64 {
        println!("{}", d[n - 1]);
        return;
    }
    let v = v - n as i64;
    // O(n^3), therefore the overall complexity is O(n^4)
    const W: usize = 510_000;
    const INF: i64 = 1 << 60;
    let mut dp = vec![INF; W];
    dp[0] = 0;
    for i in 0..n {
        let a = i + 1;
        for j in 0..W - a {
            dp[j + a] = min(dp[j + a], dp[j] + d[i]);
        }
    }
    let mut mi = INF;
    for i in 0..W {
        if i as i64 > v {
            continue;
        }
        let rest = v - i as i64;
        for j in 0..n {
            if rest % (j + 1) as i64 != 0 { continue; }
            let tmp = dp[i] + (rest / (j + 1) as i64) * d[j];
            mi = min(mi, tmp);
        }
    }
    println!("{}", mi + d[n - 1]);
}
