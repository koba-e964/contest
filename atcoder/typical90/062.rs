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

// Tags: topological-sort
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        ab: [(usize1, usize1); n],
    }
    let mut g = vec![vec![]; n];
    let mut indeg = vec![0; n];
    for i in 0..n {
        let (a, b) = ab[i];
        if a != i {
            indeg[i] += 1;
            g[a].push(i);
        }
        if b != i {
            indeg[i] += 1;
            g[b].push(i);
        }
    }
    let mut rem = n;
    let mut st = vec![];
    for i in 0..n {
        if indeg[i] <= 1 {
            st.push(i);
        }
    }
    let mut path = vec![];
    while let Some(v) = st.pop() {
        rem -= 1;
        path.push(v);
        for &w in &g[v] {
            indeg[w] -= 1;
            if indeg[w] == 1 {
                st.push(w);
            }
        }
    }
    if rem > 0 {
        puts!("-1\n");
        return;
    }
    path.reverse();
    for i in 0..n {
        puts!("{}\n", path[i] + 1);
    }
}
