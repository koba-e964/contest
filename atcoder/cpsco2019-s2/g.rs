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
 * Verified by https://atcoder.jp/contests/pakencamp-2019-day3/submissions/9253305
 */
struct UnionFind { disj: Vec<usize>, rank: Vec<usize> }

impl UnionFind {
    fn new(n: usize) -> Self {
        let disj = (0..n).collect();
        UnionFind { disj: disj, rank: vec![1; n] }
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
        self.rank[y] += self.rank[x];
    }
    #[allow(unused)]
    fn is_same_set(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    #[allow(unused)]
    fn size(&mut self, x: usize) -> usize {
        let x = self.root(x);
        self.rank[x]
    }
}

// Tags: minimum-spanning-tree, matroid
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize, m: usize,
        uvcw: [(usize1, usize1, i32, chars); m],
        q: usize,
        a: [i64; q],
    }
    let mut qs = vec![];
    for i in 0..q {
        qs.push((a[i], i));
    }
    qs.sort();
    let mut eo = vec![];
    let mut ex = vec![];
    for &(u, v, c, ref w) in &uvcw {
        if c == 0 {
            eo.push((w.into_iter().collect::<String>().parse::<i64>().unwrap(), u, v));
        } else {
            ex.push((u, v));
        }
    }
    eo.sort();
    let mut uf = UnionFind::new(n);
    for &(u, v) in &ex {
        uf.unite(u, v);
    }
    let mut nec = vec![];
    let mut rem = vec![];
    let mut tot = 0;
    let mut inc = UnionFind::new(n);
    let mut conn = n;
    for &(c, v, w) in &eo {
        if uf.is_same_set(v, w) {
            rem.push((c, v, w));
        } else {
            uf.unite(v, w);
            if !inc.is_same_set(v, w) {
                nec.push((c, v, w));
                tot += c;
                inc.unite(v, w);
                conn -= 1;
            }
        }
    }
    // eprintln!("rem = {:?}, nec = {:?}, tot = {}", rem, nec, tot);
    let mut ans = vec![-1; q];
    let mut pos = 0;
    for &(a, idx) in &qs {
        while pos < rem.len() && rem[pos].0 <= a {
            let (c, v, w) = rem[pos];
            if !inc.is_same_set(v, w) {
                inc.unite(v, w);
                conn -= 1;
                tot += c;
            }
            pos += 1;
        }
        ans[idx] = (conn as i64 - 1) * a + tot;
    }
    for i in 0..q {
        puts!("{}\n", ans[i]);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
