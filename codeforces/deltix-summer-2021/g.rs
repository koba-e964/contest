use std::cmp::*;
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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Bisect<T> {
    fn lower_bound(&self, val: &T) -> usize;
    fn upper_bound(&self, val: &T) -> usize;
}

impl<T: Ord> Bisect<T> for [T] {
    fn lower_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] >= val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
    fn upper_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] > val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
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

// Port from https://satanic0258.github.io/snippets/data-structure/SegmentMap.html
// Verified by: https://yukicoder.me/submissions/701257
#[derive(Clone, Debug, Default)]
struct Segs {
    s: std::collections::BTreeMap<i64, i64>,
}

impl Segs {
    fn new() -> Self { Default::default() }
    #[allow(unused)]
    fn get(&self, x: i64) -> Option<(i64, i64)> {
        if let Some((&l, &r)) = self.s.range(..=x).rev().next() {
            if x < r {
                Some((l, r))
            } else {
                None
            }
        } else {
            None
        }
    }
    // adds [l, r).
    fn add(&mut self, mut l: i64, mut r: i64) {
        assert!(l <= r);
        if l == r { return; }
        fn deref((&x, &y): (&i64, &i64)) -> (i64, i64) { (x, y) }
        let mut p = self.s.range(..l).rev().next().map(deref);
        if p.is_none() {
            p = self.s.iter().next().map(deref);
        }
        while let Some((a, b)) = p {
            if a > r { break; }
            if b >= l {
                l = std::cmp::min(l, a);
                r = std::cmp::max(r, b);
                self.s.remove(&a);
            }
            p = self.s.range(a + 1..).next().map(deref);
        }
        self.s.insert(l, r);
    }
    // removes [l, r).
    #[allow(unused)]
    fn remove(&mut self, l: i64, r: i64) {
        assert!(l <= r);
        if l == r { return; }
        fn deref((&x, &y): (&i64, &i64)) -> (i64, i64) { (x, y) }
        let mut p = self.s.range(..l).rev().next().map(deref);
        if p.is_none() {
            p = self.s.iter().next().map(deref);
        }
        let mut tl = std::i64::MAX;
        let mut tr = std::i64::MIN;
        while let Some((a, b)) = p {
            if a > r { break; }
            if b >= l {
                tl = std::cmp::min(tl, a);
                tr = std::cmp::max(tr, b);
                self.s.remove(&a);
            }
            p = self.s.range(a + 1..).next().map(deref);
        }
        if tl < l { self.s.insert(tl, l); }
        if r < tr { self.s.insert(r, tr); }
    }
    #[allow(unused)]
    fn each<F: FnMut(i64, i64)>(&self, mut f: F) {
        for (&x, &y) in &self.s { f(x, y); }
    }
}

fn dfs(
    used: &[(i64, i64)], l: i64, r: i64,
    segs: &mut Vec<(i64, i64)>,
    g: &mut Vec<Vec<usize>>,
) {
    assert_ne!(used.len(), 0);
    if used.len() == 1 && used[0].0 <= l && used[0].1 >= r {
        segs.push((l, r));
        g.push(vec![]);
        return;
    }
    assert!(r - l >= 2);
    let offset = segs.len();
    let mid = (l + r) / 2;
    let idx = used.lower_bound(&(mid, 0));
    dfs(&used[..idx], l, mid, segs, g);
    let mid_offset = segs.len();
    let idx = used.lower_bound(&(mid + 1, 0)) - 1;
    dfs(&used[idx..], mid, r, segs, g);
    // induce
    let mut pos = mid_offset;
    let diff = mid - l;
    for i in offset..mid_offset {
        let (a, b) = segs[i];
        while pos < segs.len() {
            let (c, d) = segs[pos];
            if max(a, c - diff) < min(b, d - diff) {
                g[i].push(pos);
                g[pos].push(i);
            }
            if d - diff < b {
                pos += 1;
            } else {
                break;
            }
        }
    }
}

fn construct_graph(n: usize, used: &[(i64, i64)]) -> (Vec<(i64, i64)>, Vec<Vec<usize>>) {
    let mut segs = vec![];
    let mut g = vec![];
    dfs(used, 0, 1 << n, &mut segs, &mut g);
    (segs, g)
}

// The author read the tutorial before implementing this.
// Tags: segment-splitting, constructing-smart-graphs, reverse-order
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize,
        qs: [(String, i64, i64); m],
    }
    let mut segs = Segs::new();
    segs.add(0, 1 << n);
    let mut used = vec![];
    for &(ref s, l, r) in &qs {
        if s == "block" {
            segs.remove(l, r + 1);
            used.push((l, r + 1));
        }
    }
    segs.each(|x, y| used.push((x, y)));
    used.sort();
    let (slots, g) = construct_graph(n, &used);
    let m = slots.len();
    let mut uf = UnionFind::new(m);
    let mut free = vec![false; m];
    segs.each(|l, r| {
        let lo = slots.lower_bound(&(l, 0));
        let hi = slots.upper_bound(&(r, 0));
        for j in lo..hi {
            free[j] = true;
            for &w in &g[j] {
                if free[w] {
                    uf.unite(j, w);
                }
            }
        }
    });
    let mut ans = vec![];
    for &(ref s, l, r) in qs.iter().rev() {
        if s == "block" {
            let lo = slots.lower_bound(&(l, 0));
            let hi = slots.upper_bound(&(r + 1, 0));
            for j in lo..hi {
                free[j] = true;
                for &w in &g[j] {
                    if free[w] {
                        uf.unite(j, w);
                    }
                }
            }
        } else {
            let a = slots.lower_bound(&(l + 1, 0)) - 1;
            let b = slots.lower_bound(&(r + 1, 0)) - 1;
            ans.push(if uf.is_same_set(a, b) { 1u8 } else { 0u8 });
        }
    }
    ans.reverse();
    for a in ans {
        puts!("{}\n", a);
    }
}
