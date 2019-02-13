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
 * Verified by https://atcoder.jp/contests/keyence2019/submissions/4071067
 */
struct UnionFind { disj: Vec<usize>, rank: Vec<usize> }

impl UnionFind {
    fn new(n: usize) -> Self {
        let disj = (0..n).collect();
        UnionFind { disj: disj, rank: vec![0; n] }
    }
    fn root(&mut self, x: usize) -> usize {
        if x != self.disj[x] {
            let par = self.disj[x];
            let r = self.root(par);
            self.disj[x] = r;
        }
        self.disj[x]
    }
    fn unite(&mut self, x: usize, y: usize) {
        let mut x = self.root(x);
        let mut y = self.root(y);
        if x == y { return }
        if self.rank[x] > self.rank[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.disj[x] = y;
        self.rank[y] = std::cmp::max(self.rank[y], self.rank[x] + 1);
    }
    #[allow(unused)]
    fn is_same_set(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        k: usize,
        q: usize,
        s: [chars; n],
        query: [[usize1; 4]; q],
    }
    let mut acc = vec![vec![0i32; n + 1]; n + 1];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '.' { continue; }
            let x = max(i + 1, k) - k;
            let y = max(j + 1, k) - k;
            acc[i + 1][j + 1] += 1;
            acc[i + 1][y] -= 1;
            acc[x][j + 1] -= 1;
            acc[x][y] += 1;
        }
    }
    for i in 0..n {
        for j in 0..n + 1 {
            acc[i + 1][j] += acc[i][j];
        }
    }
    for i in 0..n + 1 {
        for j in 0..n {
            acc[i][j + 1] += acc[i][j];
        }
    }
    let mut uf = UnionFind::new(n * n);
    for i in 0..n - k + 1 {
        for j in 0..n - k {
            if acc[i][j] != 0 || acc[i][j + 1] != 0 { continue; }
            uf.unite(i * n + j, i * n + j + 1);
        }
    }
    for i in 0..n - k {
        for j in 0..n - k + 1 {
            if acc[i][j] != 0 || acc[i + 1][j] != 0 { continue; }
            uf.unite(i * n + j, i * n + n + j);
        }
    }
    for rc in query {
        let (r1, c1) = (rc[0], rc[1]);
        let (r2, c2) = (rc[2], rc[3]);
        puts!("{}\n", if uf.is_same_set(r1 * n + c1, r2 * n + c2) { "Yes" } else { "No" });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
