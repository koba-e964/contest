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

// Tags: inversion-number, fps
fn main() {
    input! {
        n: usize, k: usize,
        _a: [i64; n],
    }
    let lim = n * (n - 1) / 2;
    let mut dp = vec![0; lim + 1];
    dp[0] = 1;
    const MOD: i64 = 1_000_000_007;
    for i in 1..n {
        // *= 1 + x + .. + x^i
        for j in (i + 1..lim + 1).rev() {
            dp[j] = (dp[j] - dp[j - i - 1] + MOD) % MOD;
        }
        for j in 1..lim + 1 {
            dp[j] = (dp[j] + dp[j - 1]) % MOD;
        }
    }
    let mut ans = 0;
    for i in 0..k + 1 {
        ans = (ans + dp[i]) % MOD;
    }
    println!("{}", ans);
}
