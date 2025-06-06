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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Tags: partition-numbers, dp
fn main() {
    input! {
        q: usize,
        tnk: [(i32, usize, usize); q],
    }
    const MOD: i64 = 998_244_353;
    const W: usize = 4001;
    let mut dp = vec![vec![0; W]; W];
    for i in 0..W {
        dp[0][i] = 1;
    }
    for i in 1..W {
        for j in 1..i + 1 {
            dp[i][j] = (dp[i - j][j] + dp[i][j - 1]) % MOD;
        }
        for j in i + 1..W {
            dp[i][j] = dp[i][j - 1];
        }
    }
    for (_, n, k) in tnk {
        println!("{}", dp[n][k]);
    }
}
