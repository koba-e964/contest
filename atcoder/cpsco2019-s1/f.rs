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
/*
min (A_i - B_i * |p_i - t_i|) を最大にする順列 p を求めたい。
forall i. A_i - B_i * |p_i - t_i| >= x なる p が存在するかどうか判定したい。
p_i の存在範囲が与えられるので、実現する p があるかどうか判定すればよい。
それはできる。
 */

fn realizable(ranges: &[(i64, i64)]) -> bool {
    let n = ranges.len();
    let nn = n as i64;
    let mut app = vec![vec![]; n];
    for &(l, r) in ranges {
        let l = max(l, 0) as usize;
        app[l].push(min(r, nn - 1));
    }
    let mut que = BinaryHeap::new();
    for i in 0..n {
        for &v in &app[i] {
            que.push(-v);
        }
        let mut found = false;
        while let Some(x) = que.pop() {
            let x = -x;
            if x < i as i64 {
                continue;
            }
            found = true;
            break;
        }
        if !found {
            return false;
        }
    }
    true
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        tab: [(usize1, i64, i64); n],
    }
    let mut pass = -(n as i64) * 50001;
    let mut fail = 1_000_000_001;
    while fail - pass > 1 {
        let mid = (fail - pass) / 2 + pass;
        let mut ranges = vec![(0, 0); n];
        let mut ok = true;
        for i in 0..n {
            let (t, a, b) = tab[i];
            if a < mid {
                ok = false;
                break;
            }
            let diff = (a - mid) / b;
            ranges[i] = (t as i64 - diff, t as i64 + diff);
        }
        if ok && realizable(&mut ranges) {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    puts!("{}\n", pass);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
