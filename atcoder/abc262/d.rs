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
        n: usize,
        a: [usize; n],
    }
    const MOD: i64 = 998_244_353;
    let mut ans = 0;
    for k in 1..n + 1 {
        let mut dp = vec![vec![0; k]; k + 1];
        dp[0][0] = 1;
        for i in 0..n {
            let r = a[i] % k;
            for j in (0..k).rev() {
                for l in 0..k {
                    dp[j + 1][(l + r) % k] += dp[j][l];
                    dp[j + 1][(l + r) % k] %= MOD;
                }
            }
        }
        ans = (ans + dp[k][0]) % MOD;
    }
    println!("{}", ans);
}
