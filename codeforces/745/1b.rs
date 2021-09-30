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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    let debug = false;
    input!(n: usize, m: usize, k: usize, mo: i64);
    const W: usize = 200;
    let mut comb = vec![vec![0; W + 1]; W + 1];
    let mut fac = vec![0; W + 1];
    comb[0][0] = 1 % mo;
    fac[0] = 1 % mo;
    for i in 1..W + 1 {
        for j in 0..i + 1 {
            comb[i][j] = comb[i - 1][j];
            if j > 0 {
                comb[i][j] = (comb[i][j] + comb[i - 1][j - 1]) % mo;
            }
        }
        fac[i] = fac[i - 1] * i as i64 % mo;
    }
    for i in 0..5 {
        eprintln!("{:?}", &comb[i][..5]);
    }
    let mut dp = vec![vec![0i64; k + 1]; n + 1];
    dp[1][1] = 1 % mo;
    for i in 2..n + 1 {
        dp[i][1] = fac[i];
    }
    dp[0][0] = 1 % mo;
    let mut ep = vec![vec![0i64; k + 1]; n + 1];
    for l in 2..m + 1 {
        for i in 0..n + 1 {
            for j in 0..k + 1 {
                ep[i][j] = 0;
            }
        }
        ep[0][0] = 1 % mo;
        for i in 0..n + 1 {
            for u in i + 1..n + 1 {
                ep[u][0] = (ep[u][0] + dp[i][0] * dp[u - i - 1][0] % mo
                    * comb[u - 1][i]) % mo;
            }
            for j in 1..min(k, i) + 1 {
                if dp[i][j] == 0 {
                    continue;
                }
                for u in i + 1..n + 1 {
                    let coef = dp[i][j] * comb[u - 1][i] % mo;
                    for v in j..min(u, k) + 1 {
                        let mut tmp = ep[u][v];
                        tmp += coef * dp[u - i - 1][v - j] *
                        // j == 0 handled here
                            if v == j {
                                2
                            } else {
                                1
                            } % mo;
                        ep[u][v] = if tmp >= mo { tmp - mo } else { tmp };
                    }
                }
            }
        }
        std::mem::swap(&mut dp, &mut ep);
        if debug {
            eprintln!("dp[{}] = {:?}", l, dp);
        }
    }
    println!("{}", dp[n][k]);
}
