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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        n: usize, m: usize,
        uv: [(usize1, usize1); m],
        a: [[i64; n]; n - 1],
    }
    let mut g = vec![0; n];
    for (u, v) in uv {
        g[u] |= 1 << v;
        g[v] |= 1 << u;
    }
    const INF: i64 = 1 << 50;
    let mut dp = vec![-INF; 1 << n];
    dp[1] = 0;
    for bits in 1..(1 << n) - 1 {
        let me = dp[bits];
        let bc = bits.count_ones() as usize;
        for i in 0..n {
            if (bits & 1 << i) != 0 { continue; }
            if (bits & g[i]) != 0 {
                dp[bits | 1 << i] = max(dp[bits | 1 << i], me + a[bc - 1][i]);
            }
        }
    }
    println!("{}", dp[(1 << n) - 1]);
}
