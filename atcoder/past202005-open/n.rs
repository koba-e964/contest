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

// The author read the editorial.
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize,
        txy: [(i32, usize1, usize)],
    }
    let mut a: Vec<_> = (0..n).collect();
    let mut inv = BTreeSet::new();
    for (t, x, y) in txy {
        if t == 1 {
            a.swap(x, x + 1);
            let mut cand = vec![x];
            if x > 0 {
                cand.push(x - 1);
            }
            if x + 2 < n {
                cand.push(x + 1);
            }
            for c in cand {
                if a[c] < a[c + 1] {
                    inv.remove(&c);
                } else {
                    inv.insert(c);
                }
            }
        } else {
            assert_eq!(t, 2);
            loop {
                let cand: Vec<_> = inv.range(x..y - 1).cloned().collect();
                if cand.is_empty() { break; }
                for c in cand {
                    if a[c] > a[c + 1] {
                        a.swap(c, c + 1);
                        inv.remove(&c);
                        let mut nc = vec![];
                        if c > 0 {
                            nc.push(c - 1);
                        }
                        if c + 2 < n {
                            nc.push(c + 1);
                        }
                        for d in nc {
                            if a[d] < a[d + 1] {
                                inv.remove(&d);
                            } else {
                                inv.insert(d);
                            }
                        }
                    }
                }
            }
        }
    }
    for i in 0..n {
        puts!("{}{}", a[i] + 1, if i + 1 == n { "\n" } else { " " });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
