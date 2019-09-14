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

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
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

/**
 * Segment Tree. This data structure is useful for fast folding on intervals of an array
 * whose elements are elements of monoid I. Note that constructing this tree requires the identity
 * element of I and the operation of I.
 * Verified by: yukicoder No. 259 (http://yukicoder.me/submissions/100581)
 *              AGC015-E (http://agc015.contest.atcoder.jp/submissions/1461001)
 */
struct SegTree<I, BiOp> {
    n: usize,
    dat: Vec<I>,
    op: BiOp,
    e: I,
}

impl<I, BiOp> SegTree<I, BiOp>
    where BiOp: Fn(I, I) -> I,
          I: Copy {
    pub fn new(n_: usize, op: BiOp, e: I) -> Self {
        let mut n = 1;
        while n < n_ { n *= 2; } // n is a power of 2
        SegTree {n: n, dat: vec![e; 2 * n - 1], op: op, e: e}
    }
    /* ary[k] <- v */
    pub fn update(&mut self, idx: usize, v: I) {
        let mut k = idx + self.n - 1;
        self.dat[k] = v;
        while k > 0 {
            k = (k - 1) / 2;
            self.dat[k] = (self.op)(self.dat[2 * k + 1], self.dat[2 * k + 2]);
        }
    }
    /* [a, b) (note: half-inclusive)
     * http://proc-cpuinfo.fixstars.com/2017/07/optimize-segment-tree/ */
    pub fn query(&self, mut a: usize, mut b: usize) -> I {
        let mut left = self.e;
        let mut right = self.e;
        a += self.n - 1;
        b += self.n - 1;
        while a < b {
            if (a & 1) == 0 {
                left = (self.op)(left, self.dat[a]);
            }
            if (b & 1) == 0 {
                right = (self.op)(self.dat[b - 1], right);
            }
            a = a / 2;
            b = (b - 1) / 2;
        }
        (self.op)(left, right)
    }
}

const INF: i64 = 1 << 50;

fn get_max<T: Ord + Copy>(set: &BTreeSet<T>) -> Option<T> {
    set.iter().rev().next().cloned()
}

fn update_color_indexed_segtree(
    n: usize,
    nearest: &mut [(i64, usize)],
    events: &[(i64, i8, usize, usize)],
    lr: &[(i64, i64)],
) {
    let mut st = SegTree::new(n, max, (-INF, 0));
    let mut pool = vec![BTreeSet::new(); n];
    for &(pos, kind, col, idx) in events {
        if kind == 0 { // end
            let ma = st.query(0, col);
            let ma = max(ma, st.query(col + 1, n));
            let dist = pos - ma.0;
            if dist < INF {
                nearest[col] = min(nearest[col], (dist, ma.1));
            }
            assert!(pool[col].remove(&(lr[idx].0, idx)));
            let ma = get_max(&pool[col]).unwrap_or((-INF, 0));
            st.update(col, ma);
        } else { // start
            let val = st.query(col, col + 1);
            st.update(col, max(val, (pos, idx)));
            assert_eq!(pos, lr[idx].0);
            pool[col].insert((pos, idx));
        }
    }
}

struct Top2 {
    top2: [(i64, usize); 2],
}

impl Top2 {
    fn new() -> Self {
        Top2 {
            top2: [(-INF, !0), (-INF, !1)],
        }
    }

    fn has_container(&self, r: i64, col: usize) -> Option<usize> {
        for &(r_p, idx) in &self.top2 {
            if r <= r_p && idx != col {
                return Some(idx);
            }
        }
        None
    }

    fn update(&mut self, r: i64, col: usize) {
        let top2 = &mut self.top2;
        if top2[0].1 == col {
            top2[0] = max(top2[0], (r, col));
        } else {
            top2[1] = max(top2[1], (r, col));
            if top2[0] < top2[1] {
                top2.swap(0, 1);
            }
        }
    }
}

fn boruvka_step(
    uf: &mut UnionFind,
    lr: &[(i64, i64)],
    mapped_lr: &[(usize, usize)],
    coord: &[i64],
    conn: &mut usize,
) -> i64 {
    let n = lr.len();
    let m = coord.len();
    let mut nearest = vec![(INF, 0); n];
    // coord-indexed segtree
    let mut st = SegTree::new(m, min, (INF, 0));
    let mut cols = vec![vec![]; n];
    for i in 0..n {
        let r = uf.root(i);
        cols[r].push(i);
    }
    for col in 0..n {
        for &idx in &cols[col] {
            let (l, r) = mapped_lr[idx];
            let mi = st.query(l, r);
            nearest[col] = min(nearest[col], mi);
        }
        for &idx in &cols[col] {
            let (li, _ri) = mapped_lr[idx];
            let (l, r) = lr[idx];
            let val = st.query(li, li + 1);
            st.update(li, min(val, (r - l, col)));
        }
    }
    let mut st = SegTree::new(m, min, (INF, 0));
    for col in (0..n).rev() {
        for &idx in &cols[col] {
            let (l, r) = mapped_lr[idx];
            let mi = st.query(l, r);
            nearest[col] = min(nearest[col], mi);
        }
        for &idx in &cols[col] {
            let (li, _ri) = mapped_lr[idx];
            let (l, r) = lr[idx];
            let val = st.query(li, li + 1);
            st.update(li, min(val, (r - l, col)));
        }
    }

    // color-indexed segtree
    let mut events = vec![];
    for i in 0..n {
        events.push((lr[i].0, 1, uf.root(i), i));
        events.push((lr[i].1, 0, uf.root(i), i));
    }
    events.sort();
    update_color_indexed_segtree(n, &mut nearest, &events, &lr);
    // reverse
    events.reverse();
    for v in events.iter_mut() {
        v.1 = 1 - v.1;
        v.0 = -v.0;
    }
    let lr_neg: Vec<_> = lr.iter().map(|&(l, r)| (-r, -l)).collect();
    update_color_indexed_segtree(n, &mut nearest, &events, &lr_neg);

    // included
    let mut events = vec![];
    for i in 0..n {
        events.push((lr[i].0, -lr[i].1, uf.root(i)));
    }
    events.sort();
    let mut top2 = Top2::new();
    for &(l, r, col) in &events {
        let r = -r;
        if let Some(to) = top2.has_container(r, col) {
            nearest[col] = min(nearest[col], (r - l, to));
        }
        top2.update(r, col);
    }
    
    // debugln!("{:?}", nearest);
    let mut tot = 0;
    for i in 0..n {
        if nearest[i].0 < INF / 2 {
            assert_eq!(uf.root(i), i);
        }
    }
    for i in 0..n {
        if nearest[i].0 < INF / 2 {
            let v = nearest[i].1;
            if !uf.is_same_set(i, v) {
                uf.unite(i, v);
                tot += nearest[i].0;
                *conn -= 1;
            }
        }
    }
    tot
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        lr: [(i64, i64); n],
    }
    // coord-comp
    let mut coord = vec![];
    for &(l, r) in &lr {
        coord.extend_from_slice(&[l, r]);
    }
    coord.sort();
    coord.dedup();
    let mut mapped_lr = vec![];
    for &(l, r) in &lr {
        let l = coord.binary_search(&l).unwrap();
        let r = coord.binary_search(&r).unwrap();
        mapped_lr.push((l, r));
    }
    let mut uf = UnionFind::new(n);
    let mut tot = 0;
    let mut conn = n;
    loop {
        let oldconn = conn;
        tot += boruvka_step(&mut uf, &lr, &mapped_lr, &coord, &mut conn);
        if conn == 1 || oldconn == conn {
            break;
        }
    }
    puts!("{}\n", if conn > 1 { -1 } else { tot });
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
