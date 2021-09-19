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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn calc(a: &[Vec<usize>]) -> i64 {
    let n = a.len();
    let mut indeg = vec![0; n];
    let mut g = vec![vec![]; n];
    for i in 0..n {
        indeg[i] = a[i].len();
        for &w in &a[i] {
            g[w].push(i);
        }
    }
    let mut que = BinaryHeap::new();
    for i in 0..n {
        if indeg[i] == 0 {
            que.push(Reverse(i));
        }
    }
    let mut tmp = vec![];
    let mut rem = n;
    let mut ans = 0;
    while !que.is_empty() {
        while let Some(Reverse(v)) = que.pop() {
            rem -= 1;
            for &w in &g[v] {
                indeg[w] -= 1;
                if indeg[w] == 0 {
                    if w < v {
                        tmp.push(w);
                    } else {
                        que.push(Reverse(w));
                    }
                }
            }
        }
        for v in tmp.drain(..) {
            que.push(Reverse(v));
        }
        ans += 1;
    }
    if rem == 0 {
        ans
    } else {
        -1
    }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input!(a: [[[usize1]]]);
    for a in a {
        puts!("{}\n", calc(&a));
    }
}
