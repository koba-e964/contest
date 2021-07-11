#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
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

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

// Tags: bit-dp
fn main() {
    input! {
        h: usize, w: usize,
        c: [chars; h],
    }
    let n = h * w;
    let mut g = vec![vec![]; n];
    for i in 0..h - 1 {
        for j in 0..w {
            let v = i * w + j;
            g[v].push(v + w);
            g[v + w].push(v);
        }
    }
    for i in 0..h {
        for j in 0..w - 1 {
            let v = i * w + j;
            g[v].push(v + 1);
            g[v + 1].push(v);
        }
    }
    let mut avail = vec![false; n];
    for i in 0..h {
        for j in 0..w {
            avail[i * w + j] = c[i][j] == '.';
        }
    }
    let mut ma = -1;
    for i in 0..n {
        if !avail[i] {
            continue;
        }
        let mut dp = vec![vec![-1; 1 << n]; n];
        for &w in &g[i] {
            if avail[w] {
                dp[w][1 << w] = 1;
            }
        }
        for bits in 1usize..1 << n {
            let c = bits.count_ones();
            if c <= 1 {
                continue;
            }
            for j in 0..n {
                let mut me = -1;
                if (bits & 1 << j) == 0 || !avail[j] {
                    continue;
                }
                let pre = bits ^ 1 << j;
                for &w in &g[j] {
                    if dp[w][pre] > 0 {
                        me.chmax(dp[w][pre] + 1);
                    }
                }
                dp[j][bits] = me;
            }
        }
        for bits in 0..1 << n {
            if dp[i][bits] >= 3 {
                ma.chmax(dp[i][bits]);
            }
        }
    }
    println!("{}", ma);
}
