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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

const MOD: i64 = 10_007;

fn main() {
    input! {
        n: usize,
        s: chars,
    }
    let mut dp = vec![[0; 8]; n + 1];
    dp[0][1] = 1;
    for i in 0..n {
        let idx = ['J', 'O', 'I'].iter().position(|&x| x == s[i]).unwrap();
        for a in 0..8 {
            for b in 0..8 {
                if (b & 1 << idx) == 0 || (a & b) == 0 {
                    continue;
                }
                dp[i + 1][b] += dp[i][a];
            }
        }
        for a in 0..8 {
            dp[i + 1][a] %= MOD;
        }
    }
    let s: i64 = dp[n].iter().sum();
    println!("{}", s % MOD);
}
