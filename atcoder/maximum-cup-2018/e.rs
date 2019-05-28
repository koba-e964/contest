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

const B: usize = 18;
const N: usize = 1 << B;

fn lca(tbl: &[Vec<usize>], dep: &[usize], mut x: usize, mut y: usize) -> usize {
    if dep[x] > dep[y] {
        std::mem::swap(&mut x, &mut y);
    }
    if dep[x] < dep[y] {
        for i in (0..B).rev() {
            if dep[y] >= dep[x] + (1 << i) {
                y = tbl[i][y];
            }
        }
    }
    if x == y {
        return x;
    }
    for i in (0..B).rev() {
        if tbl[i][x] != tbl[i][y] {
            x = tbl[i][x];
            y = tbl[i][y];
        }
    }
    tbl[0][x]
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input!(qs: [(chars, usize1, usize1)]);
    let mut dep = vec![0; N];
    let mut tbl = vec![vec![0; N]; B];
    for &(ref ty, a, b) in &qs {
        if ty[0] == 'A' {
            tbl[0][b] = a;
            for i in 1..B {
                tbl[i][b] = tbl[i - 1][tbl[i - 1][b]];
            }
            dep[b] = dep[a] + 1;
        } else {
            let l = lca(&tbl, &dep, a, b);
            let dist = dep[a] + dep[b] - 2 * dep[l];
            puts!("{}\n", dist - 1);
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
