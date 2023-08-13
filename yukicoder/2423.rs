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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        n: usize, k: usize,
        a: [i64; n],
        c: [usize1; n],
    }
    let mut acc = vec![0; 2 * n + 1];
    for i in 0..2 * n {
        acc[i + 1] = acc[i] + a[i % n];
    }
    const C: usize = 50;
    let mut ans = 0;
    let mut dp = vec![vec![0u64; 2 * n + 1]; 2 * n];
    for i in 0..2 * n {
        let u = i % n;
        dp[i][i + 1] |= 1 << c[u];
        ans = max(ans, a[u]);
    }
    for s in 2..n + 1 {
        for i in 0..2 * n - s + 1 {
            let j = i + s;
            let mut me = 0u64;
            for l in i + 1..j {
                for b in 0..C {
                    if (dp[i][l] & 1 << b) != 0 {
                        let bits = (1 << (b + k + 1)) - (1 << b);
                        let bits = bits | bits >> k;
                        if (dp[l][j] & bits) != 0 {
                            me |= 1 << b | (dp[l][j] & bits);
                        }
                    }
                }
            }
            dp[i][j] = me;
            if me != 0 {
                ans = max(ans, acc[j] - acc[i]);
            }
        }
    }
    println!("{}", ans);
}
