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

const B: usize = 31;
const C: usize = 50;

// x: max time
fn can_pack(tbl: &[Vec<i64>], k: i64, x: i64) -> bool {
    let mut rem = k;
    let mut cur_bag = tbl[0].clone();
    for i in 0..C {
        cur_bag.sort();
        if (x >> i & 1) == 1 && cur_bag.len() >= 1 {
            rem -= cur_bag.pop().unwrap();
        }
        let mut nxt = vec![];
        while let Some(mut fst) = cur_bag.pop() {
            if let Some(snd) = cur_bag.pop() {
                fst += snd;
            }
            nxt.push(fst);
        }
        if i + 1 < B {
            nxt.extend_from_slice(&&tbl[i + 1]);
        }
        cur_bag = nxt;
    }
    rem <= 0
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
        k: i64,
        ab: [(usize, i64); n],
    }
    let mut tbl = vec![vec![]; B];
    for (a, b) in ab {
        tbl[a].push(b);
    }
    let mut pass = 1 << C;
    let mut fail = 0;
    while pass - fail > 1 {
        let mid = (pass + fail) / 2;
        if can_pack(&tbl, k, mid) {
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
