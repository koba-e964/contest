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

fn powmod(x: i64, mut e: i64, m: i64) -> i64 {
    let mut sum = 1;
    let mut cur = x % m;
    while e > 0 {
        if e % 2 != 0 {
            sum = sum * cur % m;
        }
        cur = cur * cur % m;
        e /= 2;
    }
    sum
}

fn main() {
    input!(n: usize, m: usize, q: i64);
    // exp(n, m);
    let mut dp = vec![vec![0; n + 1]; n + 1];
    dp[0][0] = 1;
    let tm = |x: &mut i64| if *x >= q {
        *x -= q;
    };
    for i in 0..n + 1 {
        for j in 0..n + 1 {
            if dp[i][j] == 0 { continue; }
            let val = dp[i][j];
            // Fill with necessary
            if i + 1 <= n {
                dp[i + 1][j] += val;
                tm(&mut dp[i + 1][j]);
            }
            // Fill with unnecessary + nec
            if i + 1 <= m && j + 1 <= n {
                dp[i + 1][j + 1] += val;
                tm(&mut dp[i + 1][j + 1]);
            }
        }
    }
    let mut fac = vec![1; n + 1];
    let mut invfac = vec![1; n + 1];
    for i in 1..n + 1 {
        fac[i] = fac[i - 1] * i as i64 % q;
    }
    invfac[n] = powmod(fac[n], q - 2, q);
    for i in (0..n).rev() {
        invfac[i] = invfac[i + 1] * (i + 1) as i64 % q;
    }
    let mut acc = vec![vec![0; n + 1]; n + 1];
    for i in 0..n + 1 {
        for j in 0..n - i + 1 {
            let k = (n - i - j) / 2;
            acc[i][j] = dp[i][j] * fac[i + k] % q
                * invfac[i] % q
                * invfac[k] % q;
            if j >= 2 {
                acc[i][j] = acc[i][j] + acc[i][j - 2];
                tm(&mut acc[i][j]);
            }
        }
    }
    let mut tot = 0;
    for i in 2..m + 1 {
        if i * 2 <= n || i + 1 == n {
            tot += 1;
        }
    }
    for i in 1..n + 1 {
        for j in max(3, i)..m + 1 {
            if 2 * j <= n + i {
                tot += acc[i][n + i - 2 * j];
                tm(&mut tot);
            }
            if 2 * j + 1 <= n + i {
                tot += acc[i][n + i - 2 * j - 1];
                tm(&mut tot);
            }
        }
    }
    println!("{}", tot);
}
