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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        a: [i32; n],
        plr: [(i64, usize1, usize)],
    }
    const W: usize = 2001;
    let mut fac = vec![0; W];
    for i in 2..W {
        fac[i] = i;
    }
    for i in 2..W {
        if fac[i] != i { continue; }
        for j in 2..(W - 1) / i + 1 {
            fac[i * j] = i;
        }
    }
    let primes: Vec<_> = (2..W).filter(|&i| fac[i] == i).collect();
    let m = primes.len();
    let mut inv = vec![0; W];
    for i in 0..m {
        inv[primes[i]] = i;
    }
    let mut acc = vec![vec![0i32; n + 1]; m];
    let mut zero = vec![0i32; n + 1];
    for i in 0..n {
        for j in 0..m {
            acc[j][i + 1] = acc[j][i];
        }
        zero[i + 1] = zero[i];
        if a[i] == 0 {
            zero[i + 1] += 1;
            continue;
        }
        let mut v = a[i];
        while v > 1 {
            let p = fac[v as usize];
            v /= p as i32;
            let idx = inv[p];
            acc[idx][i + 1] += 1;
        }
    }
    for &(p, l, r) in &plr {
        assert!(1 <= p && p <= 1_000_000_000);
        assert!(l < r && r <= n);
        let mut ans = true;
        if zero[r] == zero[l] {
            let mut v = p;
            for i in 0..m {
                let mut e = 0;
                let p = primes[i] as i64;
                while v % p == 0 {
                    v /= p;
                    e += 1;
                }
                let diff = acc[i][r] - acc[i][l];
                if diff < e as i32 {
                    ans = false;
                }
            }
            if v > 1 {
                ans = false;
            }
        }
        puts!("{}\n", if ans { "Yes" } else { "NO" });
    }
}

fn main() {
    solve();
}
