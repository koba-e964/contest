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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        n: usize,
        txa: [(usize, usize, i64); n],
    }
    const INF: i64 = 1 << 50;
    let mut ev = vec![vec![]; 100_101];
    for (t, x, a) in txa {
        ev[t].push((x, a));
    }
    let mut dp = [-INF; 5];
    dp[0] = 0;
    for ev in ev {
        for (x, a) in ev {
            dp[x] += a;
        }
        let mut ep = dp;
        for i in 0..4 {
            ep[i + 1] = max(ep[i + 1], dp[i]);
            ep[i] = max(ep[i], dp[i + 1]);
        }
        dp = ep;
    }
    println!("{}", dp.iter().max().unwrap());
}
