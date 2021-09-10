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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn main() {
    input! {
        n: usize, k: usize,
        ab: [(i64, [usize1]); n],
    }
    let mut g = vec![vec![]; 2 * n + 1];
    for i in 0..n {
        let (a, ref b) = ab[i];
        g[i].push((n + i, a));
        g[n + i + 1].push((i, -a));
        for &b in b {
            g[i].push((b, 0));
        }
    }
    assert_eq!(k, 1);
    let mut dp = vec![0; 2 * n + 1];
    dp[n] = 0;
    const INF: i64 = 1 << 50;
    for i in 0..n {
        let mut mi = INF;
        for &(w, c) in &g[i] {
            mi.chmin(dp[w] + c);
        }
        dp[i] = mi;
        mi = INF;
        for &(w, c) in &g[n + i + 1] {
            mi.chmin(dp[w] + c);
        }
        dp[n + i + 1] = mi;
    }
    println!("{}", -dp[2 * n]);
}
