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

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn dfs1(v: usize, par: usize, g: &[Vec<(usize, i32)>], p: &mut [usize], ans: &mut Vec<i32>) {
    for &(w, _) in &g[v] {
        if w == par { continue; }
        dfs1(w, v, g, p, ans);
    }
    dfs2(v, g.len(), v, g, p, ans);
}

fn dfs2(v: usize, par: usize, x: usize, g: &[Vec<(usize, i32)>], p: &mut [usize], ans: &mut Vec<i32>) -> bool {
    if p[v] == x {
        return true;
    }
    for &(w, id) in &g[v] {
        if w == par { continue; }
        if dfs2(w, v, x, g, p, ans) {
            p.swap(v, w);
            ans.push(id);
            return true;
        }
    }
    false
}

// Tags: swapping-on-trees
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize,
        p: [usize1; n],
        m: usize,
        ab: [(usize1, usize1); m],
    }
    let mut p = p;
    let mut uf = UnionFind::new(n);
    let mut used = vec![];
    let mut g = vec![vec![]; n];
    for i in 0..m {
        let (a, b) = ab[i];
        if !uf.is_same_set(a, b) {
            uf.unite(a, b);
            used.push((a, b));
            g[a].push((b, i as i32 + 1));
            g[b].push((a, i as i32 + 1));
        }
    }
    let mut e = vec![vec![]; n];
    for &(a, b) in &ab {
        e[uf.root(a)].push((a, b));
    }
    let mut c = vec![vec![]; n];
    let mut d = vec![vec![]; n];
    for i in 0..n {
        c[uf.root(i)].push(i);
        d[uf.root(i)].push(p[i]);
    }
    let mut ans = vec![];
    for i in 0..n {
        if uf.root(i) == i {
            d[i].sort();
            if c[i] != d[i] {
                puts!("-1\n");
                return;
            }
            dfs1(i, n, &g, &mut p, &mut ans);
        }
    }
    puts!("{}\n", ans.len());
    putvec!(ans);
}
