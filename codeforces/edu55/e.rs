#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
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

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    input! {
        n: usize,
        c: usize,
        a: [usize; n],
    }
    let mut c_cnt = 0;
    const W: usize = 500_100;
    let mut occ = vec![Vec::new(); W];
    for i in 0 .. n {
        if a[i] == c {
            c_cnt += 1;
        } else {
            occ[a[i]].push((i, c_cnt));
        }
    }
    let mut ma = c_cnt;
    for i in 0 .. W {
        if i == c { continue; }
        let mut len = occ[i].len();
        let mut dp = vec![0i64; len];
        let mut dp2 = vec![0i64; len];
        for j in 0 .. len {
            dp[j] = occ[i][j].1 - j as i64; // c's right end
            dp2[j] = occ[i][j].1 - (j + 1) as i64; // c's left end
        }
        if false {
            eprintln!("dp[{}] = {:?}", i, dp);
            eprintln!("dp2[{}] = {:?}", i, dp2);
        }
        let mut loc_ma = 0;
        for j in 0 .. len {
            loc_ma = max(loc_ma, dp[j]);
            ma = max(ma, c_cnt - dp2[j] + loc_ma);
        }
    }
    puts!("{}\n", ma);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
