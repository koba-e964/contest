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

// https://yukicoder.me/problems/no/1668 (3)
// (隣接しているとは限らない) 同じ値を UnionFind で繋げる。その後、隣接する連結成分について、小さい方から大きい方へ辺を張る。こうしてできたグラフは DAG なので、下から w[i] = max(w[i - 1], w[i に伸びている辺の始点] + 1) として更新すれば良い。
fn main() {
    input! {
        h: usize, w: usize, n: usize,
        c: [[usize1; w]; h],
    }
    let mut uf = UnionFind::new(h * w);
    let mut pl = vec![h * w; n];
    for i in 0..h {
        for j in 0..w {
            let v = i * w + j;
            let c = c[i][j];
            if pl[c] >= h * w {
                pl[c] = v;
            } else {
                uf.unite(v, pl[c]);
            }
        }
    }
    let mut g = vec![vec![]; n];
    for i in 0..h - 1 {
        for j in 0..w {
            let h1 = c[i][j];
            let h2 = c[i + 1][j];
            if h1 < h2 {
                g[h2].push(h1);
            } else if h1 > h2 {
                g[h1].push(h2);
            }
        }
    }
    for i in 0..h {
        for j in 0..w - 1 {
            let h1 = c[i][j];
            let h2 = c[i][j + 1];
            if h1 < h2 {
                g[h2].push(h1);
            } else if h1 > h2 {
                g[h1].push(h2);
            }
        }
    }
    for i in 0..n {
        g[i].sort_unstable(); g[i].dedup();
    }
    let mut dp = vec![0; n];
    for i in 1..n {
        let mut ma = dp[i - 1];
        for &w in &g[i] {
            ma = max(ma, dp[w] + 1);
        }
        dp[i] = ma;
    }
    println!("{}", dp[n - 1] + 1);
}
