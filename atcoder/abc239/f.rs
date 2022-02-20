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

fn tree_from_degrees(a: &[usize]) -> Vec<(usize, usize)> {
    let n = a.len();
    assert!(a.iter().all(|&d| d >= 1), "Invalid degree seq: {:?}", a);
    assert_eq!(a.iter().sum::<usize>(), 2 * n - 2, "Invalid degree seq: {:?}", a);
    let mut s = vec![];
    for i in 0..n {
        s.push((a[i], i));
    }
    s.sort();
    let mut res = vec![];
    let mut st = vec![];
    while let Some((d, idx)) = s.pop() {
        if let Some((idx2, d2)) = st.pop() {
            res.push((idx, idx2));
            if d > 1 {
                st.push((idx, d - 1));
            }
            if d2 > 1 {
                st.push((idx2, d2 - 1));
            }
        } else {
            st.push((idx, d));
            continue;
        }
    }
    debug_assert_eq!(st, vec![]);
    res
}

// Tags: constructing-trees-from-degree-sequences
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize,
        d: [usize; n],
        ab: [(usize1, usize1); m],
    }
    let mut d = d;
    let mut uf = UnionFind::new(n);
    for &(a, b) in &ab {
        if uf.is_same_set(a, b) || d[a] == 0 || d[b] == 0 {
            puts!("-1\n");
            return;
        }
        d[a] -= 1;
        d[b] -= 1;
        uf.unite(a, b);
    }
    if d.iter().sum::<usize>() != 2 * (n - m - 1) {
        puts!("-1\n");
        return;
    }
    let roots: Vec<usize> = (0..n).filter(|&i| uf.root(i) == i).collect();
    let k = roots.len();
    let mut e = vec![vec![]; n];
    for i in 0..n {
        for _ in 0..d[i] {
            e[uf.root(i)].push(i);
        }
    }
    if roots.iter().any(|&r| e[r].is_empty()) {
        puts!("-1\n");
        return;
    }
    let mut d = vec![0; k];
    for i in 0..k {
        d[i] = e[roots[i]].len();
    }
    let sub = tree_from_degrees(&d);
    let mut pos = vec![0; k];
    for (x, y) in sub {
        puts!("{} {}\n", e[roots[x]][pos[x]] + 1, e[roots[y]][pos[y]] + 1);
        pos[x] += 1;
        pos[y] += 1;
    }
}
