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
        n: i64, m: usize, a: i64, b: i64,
        c: [i64; m],
    }
    let mut div = vec![];
    {
        let mut d = 1;
        while d * d <= n {
            if n % d == 0 {
                div.push(d);
                if n != d * d {
                    div.push(n / d);
                }
            }
            d += 1;
        }
        div.sort();
    }
    let k = div.len();
    const INF: i64 = 1 << 60;
    let mut dp = vec![INF; k];
    dp[0] = 0;
    for i in 0..k {
        let mut lim = INF;
        for &v in &c {
            if v % div[i] == 0 {
                lim = min(lim, v);
            }
        }
        for j in i + 1..k {
            if div[j] < lim && div[j] % div[i] == 0 {
                dp[j] = min(dp[j], dp[i] + (div[j] / div[i] - 1) * a + b);
            }
        }
    }
    println!("{}", if dp[k - 1] >= INF { -1 } else { dp[k - 1] - b });
}
