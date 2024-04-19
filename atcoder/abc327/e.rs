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
        p: [f64; n],
    }
    let mut sum = vec![0.0; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] * 0.9 + 1.0;
    }
    const INF: f64 = 1.0 / 0.0;
    let mut dp = vec![-INF; n + 1];
    dp[0] = 0.0;
    for p in p {
        let mut ep = dp.clone();
        for i in 0..n {
            let val = (sum[i] * 0.9 * dp[i] + p) / sum[i + 1];
            ep[i + 1] = ep[i + 1].max(val);
        }
        dp = ep;
    }
    let mut ans = -10000.0f64;
    for i in 1..n + 1 {
        ans = ans.max(dp[i] - 1200.0 / (i as f64).sqrt());
    }
    println!("{}", ans);
}
