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

fn squmul(a: &[Vec<i64>], b: &[Vec<i64>], mo: i64) -> Vec<Vec<i64>> {
    let n = a.len();
    let mut ret = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                ret[i][k] += a[i][j] * b[j][k];
                ret[i][k] %= mo;
            }
        }
    }
    ret
}

fn squpow(a: &[Vec<i64>], mut e: i64, mo: i64) -> Vec<Vec<i64>> {
    let n = a.len();
    let mut sum = vec![vec![0; n]; n];
    for i in 0..n { sum[i][i] = 1; }
    let mut cur = a.to_vec();
    while e > 0 {
        if e % 2 == 1 {
            sum = squmul(&sum, &cur, mo);
        }
        cur = squmul(&cur, &cur, mo);
        e /= 2;
    }
    sum
}

const MOD: i64 = 998244353;

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: i64, k: usize,
    }
    let kk = k * k;
    let mut mat = vec![vec![0; k * k * k]; k * k * k];
    for i in 0..k {
        for j in 0..k {
            for l in 0..k {
                let bef = i * kk + j * k + l;
                // ho
                mat[bef][(bef + kk) % (kk * k)] += 1;
                // mu
                mat[bef][i * kk + ((j + i) % k) * k + l] += 1;
                // ra
                mat[bef][i * kk + j * k + ((l + j) % k)] += 1;
            }
        }
    }
    let tr = squpow(&mat, n, MOD);
    let mut tot = 0;
    for i in 0..kk {
        tot = (tot + tr[0][i * k]) % MOD;
    }
    puts!("{}\n", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
