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

const B: usize = 550;

fn cmp(&(l1, r1, _): &(usize, usize, i64),
       &(l2, r2, _): &(usize, usize, i64)) -> Ordering {
    if l1 / B != l2 / B {
        return (l1 / B).cmp(&(l2 / B));
    }
    let res = r1.cmp(&r2);
    if (l1 / B) % 2 == 1 {
        res.reverse()
    } else {
        res
    }
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
        m: usize,
        h: [i64; n],
        lrx: [(usize1, usize, i64); m],
    }
    let mut lrx = lrx;
    // lrx.sort_by(cmp);
    // state
    let mut cl = 0;
    let mut cr = 0;
    for (l, r, x) in lrx {
        let x = (x + 1) / 2;
        let mut sum: i64 = h[l .. r].iter().sum();
        if x > sum * (r - l) as i64 {
            puts!("-1\n");
            continue;
        }
        let mut fail = 0;
        let mut pass: i64 = *h[l .. r].iter().max().unwrap();
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            let mut ma = 0;
            let mut c = 0;
            let mut s = 0;
            for i in l .. r {
                if h[i] <= mid {
                    s += h[i];
                    c += 1;
                } else {
                    ma = max(ma, s * c);
                    s = 0;
                    c = 0;
                }
            }
            ma = max(ma, s * c);
            if ma >= x {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        puts!("{}\n", pass);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
