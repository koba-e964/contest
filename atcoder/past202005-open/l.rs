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
    ($next:expr, [graph1; $len:expr]) => {{
        let mut g = vec![vec![]; $len];
        let ab = read_value!($next, [(usize1, usize1)]);
        for (a, b) in ab {
            g[a].push(b);
            g[b].push(a);
        }
        g
    }};
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
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
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize,
        t: [[i64]; n],
        a: [i32],
    }
    let mut t = t;
    for i in 0..n {
        t[i].reverse();
    }
    let mut s1 = BTreeSet::new();
    let mut s2 = BTreeSet::new();
    for i in 0..n {
        s1.insert((t[i][t[i].len() - 1], i, t[i].len() - 1));
        s2.insert((t[i][t[i].len() - 1], i, t[i].len() - 1));
        if t[i].len() >= 2 {
            s2.insert((t[i][t[i].len() - 2], i, t[i].len() - 2));
        }
    }
    for &a in &a {
        if a == 1 {
            let &(top, i, j) = s1.iter().rev().next().unwrap();
            puts!("{}\n", top);
            t[i].pop();
            s1.remove(&(top, i, j));
            if j > 0 {
                s1.insert((t[i][j - 1], i, j - 1));
            }
            s2.remove(&(top, i, j));
            if j > 1 {
                s2.insert((t[i][j - 2], i, j - 2));
            }
        } else {
            assert_eq!(a, 2);
            let &(top, i, j) = s2.iter().rev().next().unwrap();
            puts!("{}\n", top);
            t[i].remove(j);
            let l = t[i].len();
            if j == l {
                s1.remove(&(top, i, j));
                if j > 0 {
                    s1.insert((t[i][j - 1], i, j - 1));
                }
            } else {
                s1.remove(&(t[i][l - 1], i, l));
                s1.insert((t[i][l - 1], i, l - 1));
            }
            s2.remove(&(top, i, j));
            if l > 0 {
                s2.remove(&(t[i][l - 1], i, j + 1));
            }
            for j in 0..min(l, 2) {
                s2.insert((t[i][l - 1 - j], i, l - 1 - j));
            }
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
