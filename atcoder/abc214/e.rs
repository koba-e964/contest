use std::cmp::*;
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes.by_ref().map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        lr: [[(i64, i64)]],
    }
    for lr in lr {
        let mut coord = vec![];
        for &(l, r) in &lr {
            coord.push(l);
            coord.push(r + 1);
        }
        coord.sort(); coord.dedup();
        let m = coord.len();
        let mut dp = vec![vec![]; m];
        for &(l, r) in &lr {
            let l = coord.binary_search(&l).unwrap();
            dp[l].push(r + 1);
        }
        let mut ok = true;
        let mut last = 0;
        let mut que = BinaryHeap::<Reverse<i64>>::new();
        for i in 0..m {
            let mut d = coord[i] - last;
            while !que.is_empty() && d > 0 {
                que.pop();
                d -= 1;
            }
            if d == 0 && !que.is_empty() && que.peek().unwrap().0 <= coord[i] {
                ok = false;
                break;
            }
            for &v in &dp[i] {
                que.push(Reverse(v));
            }
            last = coord[i];
        }
        puts!("{}\n", if ok { "YES" } else { "NO" });
    }
}
