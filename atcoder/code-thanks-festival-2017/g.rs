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

fn max_indep(g: &[Vec<bool>]) -> usize {
    fn max_indep_internal(g: &[i64], mut used: usize,
                          mut remaining: i64, cur_ma: &mut usize) {
        let n = g.len();
        let mut degma = (0, 0);
        // preprocessing: eliminate all vertices with degree <= 1
        // find the max degree
        let mut updated = true;
        while updated {
            updated = false;
            degma = (0, 0);
            for i in 0..n {
                if (remaining & 1 << i) == 0 { continue; }
                let deg = (g[i] & remaining).count_ones();
                degma = max(degma, (deg + 1, i));
                if deg <= 1 {
                    used += 1;
                    remaining &= !(g[i] | 1 << i);
                    updated = true;
                }
            }
        }
        *cur_ma = max(*cur_ma, used);
        // base: there is no vertex remaining
        if remaining == 0 {
            return;
        }
        debug_assert_ne!(degma.0, 0);
        let v = degma.1;
        // We use v or don't use v.
        max_indep_internal(g, used + 1, remaining & !g[v] & !(1 << v), cur_ma);
        max_indep_internal(g, used, remaining & !(1 << v), cur_ma);
    }
    let n = g.len();
    assert!(n <= 64);
    let mut g_bits = vec![0i64; n];
    for i in 0..n {
        assert_eq!(g[i].len(), n);
        for j in 0..n {
            if g[i][j] { g_bits[i] |= 1 << j; }
        }
    }
    let mut ma = 0;
    max_indep_internal(&g_bits, 0, (1 << n) - 1, &mut ma);
    ma
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        ab: [(usize1, usize1)],
    }
    let mut g = vec![vec![false; n]; n];
    for (a, b) in ab {
        g[a][b] = true;
        g[b][a] = true;
    }
    puts!("{}\n", max_indep(&g));
}
