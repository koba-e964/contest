#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize, m: usize, k: usize, x: i32, y: i32,
        ab: [(usize1, usize1); m],
        c: [char; n],
        d: [usize1; k],
    }
    let mut g = vec![vec![]; n];
    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }
    for i in 0..n {
        g[i].push(i);
    }
    let mut sc = [[0; 3]; 3];
    for i in 0..3 {
        sc[i][i] = y;
        sc[i][(i + 1) % 3] = x;
    }
    let c: Vec<usize> = c.iter().map(|x| ['G', 'C', 'P'].iter().position(|y| y == x).unwrap()).collect();
    const INF: i32 = 1 << 30;
    let mut dp = vec![vec![-INF; n]; k + 1];
    dp[0][0] = 0;
    for i in 0..k {
        for j in 0..n {
            let mut me = -INF;
            for &w in &g[j] {
                me = max(me, dp[i][w] + sc[c[j]][c[d[i]]]);
            }
            dp[i + 1][j] = me;
        }
    }
    puts!("{}\n", dp[k].iter().max().unwrap());
}
