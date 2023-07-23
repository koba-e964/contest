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

fn main() {
    input! {
        n: i64,
        ab: [(usize, i64); 3],
    }
    const W: usize = 8_000_000;
    let mut dp = vec![0i64; W];
    for i in 1..W {
        let mut me = dp[i - 1];
        for &(a, b) in &ab {
            if i >= a {
                me = std::cmp::max(me, dp[i - a] + b);
            }
        }
        dp[i] = me;
    }
    let mut ans = 0;
    for &(a, b) in &ab {
        for i in 0..W {
            if i as i64 > n { continue; }
            let q = (n - i as i64) / a as i64;
            ans = std::cmp::max(ans, dp[i] + q * b);
        }
    }
    println!("{}", ans);
}
