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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Union-Find tree.
// Verified by https://atcoder.jp/contests/pakencamp-2019-day3/submissions/9253305
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
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize,
        ab: [(usize1, usize1); m],
    }
    let mut uf = UnionFind::new(n);
    let mut unused = vec![];
    for i in 0..m {
        let (a, b) = ab[i];
        if uf.is_same_set(a, b) {
            unused.push(i);
            continue;
        }
        uf.unite(a, b);
    }
    let mut roots = BTreeSet::new();
    for i in 0..n {
        if uf.root(i) == i {
            roots.insert(i);
        }
    }
    let mut mods = vec![];
    for e in unused {
        if roots.len() == 1 {
            break;
        }
        let last = *roots.last().unwrap();
        let (a, b) = ab[e];
        assert!(uf.is_same_set(a, b));
        let r = uf.root(a);
        let r2 = if r == last {
            roots.remove(&r);
            let val = *roots.last().unwrap();
            roots.insert(r);
            val
        } else {
            last
        };
        uf.unite(a, r2);
        mods.push((e, b, r2));
        // Which one lives, r or r2?
        if uf.root(a) == r2 {
            roots.remove(&r);
        } else {
            assert_eq!(uf.root(a), r);
            roots.remove(&r2);
        }
    }
    puts!("{}\n", mods.len());
    for (e, b, r2) in mods {
        puts!("{} {} {}\n", e + 1, b + 1, r2 + 1);
    }
}
