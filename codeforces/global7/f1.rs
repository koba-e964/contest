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

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize,
        g: [chars; n],
    }
    let mut dp = vec![vec![]; 1 << n];
    dp[0] = vec![vec![1i64]; n];
    // receive
    for bc in 1..n as u32 {
        for bits in 1usize..(1 << n) - 1 {
            if bits.count_ones() != bc { continue; }
            dp[bits] = vec![vec![0i64; 1 << bc]; n];
            for w in 0..n {
                if (bits & 1 << w) != 0 {
                    continue;
                }
                for v in 0..n {
                    if (bits & 1 << v) == 0 {
                        continue;
                    }
                    let pre = bits ^ 1 << v;
                    for s in 0..1 << (bc - 1) {
                        let val = dp[pre][v][s];
                        let e = if g[v][w] == '1' {
                            (1) << (bc - 1)
                        } else { 0 };
                        dp[bits][w][s + e] += val;
                    }
                }
            }
        }
        // reset
        for bits in 0usize..(1 << n) - 1 {
            if bits.count_ones() == bc - 1 {
                dp[bits] = vec![];
            }
        }
    }
    let mut ans = vec![0i64; 1 << (n - 1)];
    for i in 0..n {
        for s in 0..1 << (n - 1) {
            ans[s] += dp[(1 << n) - 1 - (1 << i)][i][s];
        }
    }
    for i in 0..1 << (n - 1) {
        puts!("{}{}", ans[i], if i + 1 == 1 << (n - 1) { "\n" } else { " " });
    }
}
