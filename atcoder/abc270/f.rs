use std::cmp::*;
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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
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

fn main() {
    input! {
        n: usize, m: usize,
        x: [i64; n],
        y: [i64; n],
        abz: [(usize1, usize1, i64); m],
    }
    let mut e = vec![];
    for (a, b, z) in abz {
        e.push((z, a, b));
    }
    for i in 0..n {
        e.push((x[i], i, n));
        e.push((y[i], i, n + 1));
    }
    e.sort();
    let mut mi = 1i64 << 50;
    for bits in 0..4 {
        let mut tot = 0;
        let mut uf = UnionFind::new(n + 2);
        for &(cost, x, y) in &e {
            if uf.is_same_set(x, y) { continue; }
            if (bits & 1) != 0 && y == n { continue; }
            if (bits & 2) != 0 && y == n + 1 { continue; }
            tot += cost;
            uf.unite(x, y);
        }
        let mut sz = uf.size(0);
        if uf.is_same_set(0, n) { sz -= 1; }
        if uf.is_same_set(0, n + 1) { sz -= 1; }
        if sz >= n {
            mi = min(mi, tot);
        }
    }
    println!("{}", mi);
}
