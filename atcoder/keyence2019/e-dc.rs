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

/**
 * Union-Find tree.
 * Verified by https://atcoder.jp/contests/keyence2019/submissions/4011936
 */
struct UnionFind { disj: Vec<usize>, rank: Vec<usize> }

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut disj = vec![0; n];
        for i in 0 .. n {
            disj[i] = i;
        }
        UnionFind { disj: disj, rank: vec![0; n] }
    }
    fn root(self: &mut Self, x: usize) -> usize {
        if x != self.disj[x] {
            let par = self.disj[x];
            let r = self.root(par);
            self.disj[x] = r;
        }
        return self.disj[x];
    }
    fn unite(self: &mut Self, x: usize, y: usize) {
        let mut xr = self.root(x);
        let mut yr = self.root(y);
        if self.rank[xr] > self.rank[yr] {
            std::mem::swap(&mut xr, &mut yr);
        }
        self.disj[xr] = yr;
        self.rank[yr] = max(self.rank[yr], self.rank[xr] + 1);
    }
    fn is_same_set(self: &mut Self, x: usize, y: usize) -> bool {
        return self.root(x) == self.root(y);
    }
}

const INF: i64 = 1 << 50;

// Reference: https://atcoder.jp/contests/keyence2019/submissions/4005570
// Divide and conquer
fn dc(l: usize, r: usize, a: &[i64], d: i64,
      pool: &mut Vec<(i64, usize, usize)>) {
    let n = a.len();
    if l + 1 >= r { return }
    let mid = (l + r) / 2;
    let mut mil = (INF, n);
    for i in l .. mid {
        mil = min(mil, (a[i] - d * i as i64, i));
    }
    let mut mir = (INF, n);
    for i in mid .. r {
        mir = min(mir, (a[i] + d * i as i64, i));
    }
    for i in l .. mid {
        let idx = mir.1;
        pool.push((a[i] + a[idx] + d * (idx - i) as i64, i, idx));
    }
    for i in mid .. r {
        let idx = mil.1;
        pool.push((a[idx] + a[i] + d * (i - idx) as i64, idx, i));
    }
    dc(l, mid, a, d, pool);
    dc(mid, r, a, d, pool);
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        d: i64,
        a: [i64; n],
    }
    let mut pool = Vec::new();
    dc(0, n, &a, d, &mut pool);
    pool.sort();
    let mut uf = UnionFind::new(n);
    let mut tot = 0;
    for (cost, x, y) in pool {
        if !uf.is_same_set(x, y) {
            uf.unite(x, y);
            tot += cost;
        }
    }
    puts!("{}\n", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
