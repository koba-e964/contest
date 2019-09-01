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
        n: usize,
        a: [[usize1; n - 1]; n],
    }
    let mut inv = vec![];
    let mut idx = vec![vec![0; n]; n];
    let mut pos = 0;
    for i in 0..n {
        for j in 0..i {
            inv.push((j, i));
            idx[i][j] = pos;
            idx[j][i] = pos;
            pos += 1;
        }
    }
    let m = inv.len();
    assert_eq!(m, pos);
    let mut g = vec![vec![]; m];
    let mut indeg = vec![0; m];
    for i in 0..n {
        for j in 0..n - 2 {
            let x = a[i][j];
            let y = a[i][j + 1];
            let u = idx[i][x];
            let v = idx[i][y];
            g[u].push(v);
            indeg[v] += 1;
        }
    }
    let mut days = 0;
    let mut rem = m;
    let mut que = VecDeque::new();
    for i in 0..m {
        if indeg[i] == 0 {
            que.push_back(i);
            rem -= 1;
        }
    }
    while !que.is_empty() {
        days += 1;
        let mut nq = VecDeque::new();
        while let Some(v) = que.pop_front() {
            for &w in &g[v] {
                indeg[w] -= 1;
                if indeg[w] == 0 {
                    nq.push_back(w);
                }
            }
        }
        rem -= nq.len();
        que = nq;
    }
    if rem != 0 {
        puts!("-1\n");
    } else {
        puts!("{}\n", days);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
