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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize, q: usize,
        a: [usize; n],
        qs: [usize1; q],
    }
    const W: usize = 500_100;
    let mut pr = vec![true; W];
    pr[0] = false;
    pr[1] = false;
    for i in 2..W {
        if !pr[i] { continue; }
        for j in 2..(W - 1) / i + 1 {
            pr[i * j] = false;
        }
    }
    let mut pd = vec![vec![]; W];
    for i in 2..W {
        if !pr[i] { continue; }
        for j in 1..(W - 1) / i + 1 {
            pd[i * j].push(i);
        }
    }
    let mut tot: i64 = 0;
    let mut dp = vec![0i64; W];
    let mut has = vec![false; W];
    for idx in qs {
        let x = a[idx];
        let m = pd[x].len();
        let ps = &pd[x];
        if !has[idx] {
            for bits in 0usize..1 << m {
                let mut prod = 1;
                for i in 0..m {
                    if (bits & 1 << i) != 0 { prod *= ps[i]; }
                }
                if bits.count_ones() % 2 == 0 {
                    tot += dp[prod];
                } else {
                    tot -= dp[prod];
                }
                dp[prod] += 1;
            }
        } else {
            for bits in 0usize..1 << m {
                let mut prod = 1;
                for i in 0..m {
                    if (bits & 1 << i) != 0 { prod *= ps[i]; }
                }
                dp[prod] -= 1;
                if bits.count_ones() % 2 == 0 {
                    tot -= dp[prod];
                } else {
                    tot += dp[prod];
                }
            }
        }
        has[idx] = !has[idx];
        puts!("{}\n", tot);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
