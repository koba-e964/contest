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
    ($next:expr, [graph1; $len:expr]) => {{
        let mut g = vec![vec![]; $len];
        let ab = read_value!($next, [(usize1, usize1)]);
        for (a, b) in ab {
            g[a].push(b);
            g[b].push(a);
        }
        g
    }};
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
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

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize, k: usize,
        a: [i64; n],
    }
    let mut s = vec![];
    let mut nonneg = vec![];
    let mut neg = vec![];
    for i in 0..n {
        if a[i] >= 0 {
            nonneg.push(a[i]);
        } else {
            neg.push(-a[i]);
        }
        s.push((a[i].abs(), a[i]));
    }
    nonneg.sort();
    neg.sort();
    s.sort();
    let mut ans: i64 = 1;
    const MOD: i64 = 1_000_000_007;
    let mut rem = k;
    let mut is_neg = false;
    while rem > 0 {
        let mut ma = (-1, 2);
        if rem >= 2 {
            if neg.len() >= 2 {
                ma = max(ma, (neg[neg.len() - 1] * neg[neg.len() - 2], 1));
            }
            if nonneg.len() >= 2 {
                ma = max(ma, (
                    nonneg[nonneg.len() - 1] * nonneg[nonneg.len() - 2], 0));
            }
            if neg.len() == 1 && nonneg.len() == 1 {
                ma.1 = 0;
            }
        } else {
            if !nonneg.is_empty() {
                ma.1 = 0;
            } else {
                ma.1 = 1;
            }
        }
        assert!(ma.1 <= 1);
        if ma.1 == 1 {
            if min(rem, neg.len()) >= 2 {
                ans = ans * (ma.0 % MOD) % MOD;
                neg.pop(); neg.pop();
                rem -= 2;
            } else {
                ans = ans * neg.pop().unwrap() % MOD;
                ans = (MOD - ans) % MOD;
                rem -= 1;
                is_neg = !is_neg;
            }
        } else {
            ans = ans * (nonneg.pop().unwrap() % MOD) % MOD;
            rem -= 1;
        }
    }
    if is_neg {
        ans = 1;
        let mut a = a.clone();
        a.sort(); a.reverse();
        for i in 0..k {
            ans = ans * (a[i] + MOD) % MOD;
        }
    }
    puts!("{}\n", ans);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
