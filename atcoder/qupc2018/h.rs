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

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
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

fn manacher(a: &[usize]) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let n = a.len();
    let mut r = vec![0; n];
    let mut eq = vec![];
    let mut ne = vec![];
    {
        let mut i = 0;
        let mut j = 0;
        while i < n {
            while i >= j && i + j < n && j < a[i] {
                eq.push((i - j, i + j));
                j += 1;
            }
            r[i] = j;
            assert_eq!(a[i], r[i]);
            if i >= j && i + j < n {
                ne.push((i - j, i + j));
            }
            let mut k = 1;
            while i >= k && i + k < n && k + r[i - k] < j {
                r[i + k] = r[i - k];
                k += 1;
            }
            i += k;
            j -= k;
        }
    }
    (eq, ne)
}

fn color_with_heuristics(g: &[Vec<usize>]) -> Vec<usize> {
    let n = g.len();
    let mut c = vec![0; n];
    for i in 0..n {
        let mut used = 0;
        for &w in &g[i] {
            if w < i {
                used |= 1 << c[w];
            }
        }
        let mut mex = 0;
        while (used & 1 << mex) != 0 {
            mex += 1;
        }
        c[i] = mex;
    }
    c
}

// Tags: construction, manacher, k-coloring
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        a: [usize; n],
    }
    let a: Vec<_> = a.into_iter().map(|x| (x + 1) / 2).collect();
    let mut uf = UnionFind::new(n);
    let (eq, ne) = manacher(&a);
    for &(x, y) in &eq {
        uf.unite(x, y);
    }
    let mut roots = vec![];
    for i in 0..n {
        if uf.root(i) == i {
            roots.push(i);
        }
    }
    let m = roots.len();
    let mut g = vec![vec![]; m];
    for &(x, y) in &ne {
        assert_ne!(uf.root(x), uf.root(y));
        let x = roots.binary_search(&uf.root(x)).unwrap();
        let y = roots.binary_search(&uf.root(y)).unwrap();
        g[x].push(y);
        g[y].push(x);
    }
    let col = color_with_heuristics(&g);
    for i in 0..n {
        let idx = col[roots.binary_search(&uf.root(i)).unwrap()];
        assert!(idx < 26);
        print!("{}", (b'a' + idx as u8) as char);
    }
    puts!("\n");
}
