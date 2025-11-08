#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
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
    ($next:expr, switch) => {{
        let ty = read_value!($next, u32);
        if ty == 1 {
            Ok(read_value!($next, (usize1, usize1, u32)))
        } else {
            Err((ty, read_value!($next, (usize1, usize1))))
        }
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Ported and modified from:
// - <https://blog.tiramister.net/posts/persistent-unionfind/>
// - <https://kopricky.github.io/code/DataStructure_OnGraph/partial_persistent_union_find.html>
// - <https://nyaannyaan.github.io/library/data-structure/rollback-union-find.hpp.html>
// Verified by:
// - <https://yukicoder.me/problems/no/416> <https://yukicoder.me/submissions/1129953>
// - <https://codeforces.com/contest/1444/problem/C> <https://codeforces.com/contest/1444/submission/347096667>
// - <https://atcoder.jp/contests/agc002/tasks/agc002_d> <https://atcoder.jp/contests/agc002/submissions/70649128>
struct PartiallyPersistentUnionFind {
    history: Vec<(usize, usize)>,
    par: Vec<(PPUFTime, i32)>,
    sz: Vec<Vec<(PPUFTime, i32)>>,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct PPUFTime(i32);

impl PartiallyPersistentUnionFind {
    pub fn new(n: usize) -> Self {
        let mut par = Vec::with_capacity(n);
        let mut sz = vec![vec![]; n];
        for i in 0..n {
            par.push((PPUFTime(i32::max_value()), i as i32));
            sz[i].push((PPUFTime(-1), 1));
        }
        Self {
            history: vec![],
            par,
            sz,
        }
    }
    pub fn find(&self, mut u: usize, time: PPUFTime) -> usize {
        while self.par[u].0 <= time { u = self.par[u].1 as usize; }
        u
    }
    #[allow(unused)]
    pub fn same(&self, u: usize, v: usize, time: PPUFTime) -> bool {
        self.find(u, time) == self.find(v, time)
    }
    pub fn unite(&mut self, mut u: usize, mut v: usize) -> bool {
        let time = PPUFTime(self.history.len() as i32 + 1);
        u = self.find(u, time);
        v = self.find(v, time);
        if u == v { return false; }
        if self.sz[u].last().unwrap().1 < self.sz[v].last().unwrap().1 {
            std::mem::swap(&mut u, &mut v);
        }
        self.par[v] = (time, u as i32);
        let whole = self.sz[u].last().unwrap().1 + self.sz[v].last().unwrap().1;
        self.sz[u].push((time, whole));
        self.history.push((u, v));
        true
    }
    #[allow(dead_code)]
    fn size(&self, x: usize, t: PPUFTime) -> usize {
        let x = self.find(x, t);
        let idx = self.sz[x].binary_search(&(t, i32::max_value())).err().unwrap() - 1;
        self.sz[x][idx].1 as usize
    }
    pub fn now(&self) -> PPUFTime {
        PPUFTime(self.history.len() as i32)
    }
    #[allow(dead_code)]
    pub fn rollback(&mut self, time: PPUFTime) {
        let time = time.0 as usize;
        while self.history.len() > time {
            let (u, v) = self.history.pop().unwrap();
            let _ = self.sz[u].pop();
            self.par[v] = (PPUFTime(i32::max_value()), v as i32);
        }
    }
}

// Ported and modified from:
// - <https://ei1333.github.io/luzhiled/snippets/other/offline-dynamic-connectivity.html>
struct OfflineDynamicConnectivity<T, Op> {
    n: usize,
    q: usize,
    pend: Vec<((usize, usize), (usize, usize))>,
    cnt: std::collections::HashMap<(usize, usize), u32>,
    appear: std::collections::HashMap<(usize, usize), usize>,
    assoc: Vec<T>,
    ops: Vec<Op>,
}

impl<T: Clone + Default> OfflineDynamicConnectivity<T, Op> {
    pub fn new(n: usize, q: usize) -> Self {
        Self {
            n,
            q,
            pend: vec![],
            cnt: Default::default(),
            appear: Default::default(),
            assoc: vec![Default::default(); n],
            ops: vec![],
        }
    }
    pub fn insert(&mut self, idx: usize, s: usize, t: usize) {
        let e = (s.min(t), s.max(t));
        let old = self.cnt.get(&e).cloned().unwrap_or(0);
        self.cnt.insert(e, old + 1);
        if old == 0 {
            self.appear.insert(e, idx);
        }
        self.q = self.q.max(idx + 1);
    }
    pub fn erase(&mut self, idx: usize, s: usize, t: usize) {
        let e = (s.min(t), s.max(t));
        let old = self.cnt[&e];
        assert!(old >= 1);
        self.cnt.insert(e, old - 1);
        if old == 1 {
            let ap = self.appear[&e];
            self.pend.push(((ap, idx), e));
        }
    }
    pub fn run(mut self,
        mut f: impl FnMut(&PartiallyPersistentUnionFind, usize, &[T]),
        mut with_unite: impl FnMut(&PartiallyPersistentUnionFind, usize, usize, usize, bool, &mut [T]) -> Op,
        mut with_same: impl FnMut(&PartiallyPersistentUnionFind, usize, usize, &mut [T]) -> Op,
        mut with_erase: impl FnMut(&PartiallyPersistentUnionFind, &mut [T], Op),
    ) {
        let seg = self.build();
        let mut uf = PartiallyPersistentUnionFind::new(self.n);
        self.run_inner(&mut f, &mut with_unite, &mut with_same, &mut with_erase, 0, &mut uf, &seg);
    }
    pub fn run_inner(&mut self,
        f: &mut impl FnMut(&PartiallyPersistentUnionFind, usize, &[T]),
        with_unite: &mut impl FnMut(&PartiallyPersistentUnionFind, usize, usize, usize, bool, &mut [T]) -> Op,
        with_same: &mut impl FnMut(&PartiallyPersistentUnionFind, usize, usize, &mut [T]) -> Op,
        with_erase: &mut impl FnMut(&PartiallyPersistentUnionFind, &mut [T], Op),
        k: usize,
        uf: &mut PartiallyPersistentUnionFind,
        seg: &[Vec<(usize, (usize, usize))>],
    ) {
        let start = uf.now();
        for &(idx, (x, y)) in &seg[k] {
            let now = uf.now();
            let rx = uf.find(x, now);
            let ry = uf.find(y, now);
            let united = uf.unite(x, y);
            let r = uf.find(rx, uf.now());
            let flipped = r == ry;
            let op = if united {
                with_unite(&uf, idx, r, rx + ry - r, flipped, &mut self.assoc)
            } else {
                with_same(&uf, idx, r, &mut self.assoc)
            };
            self.ops.push(op);
        }
        let segsz = (seg.len() + 1) / 2;
        if k < segsz - 1 {
            self.run_inner(f, with_unite, with_same, with_erase, 2 * k + 1, uf, seg);
            self.run_inner(f, with_unite, with_same, with_erase, 2 * k + 2, uf, seg);
        } else if k - (segsz - 1) < self.q {
            let query_index = k - (segsz - 1);
            f(&uf, query_index, &self.assoc);
        }
        for _ in seg[k].iter().rev() {
            let op = self.ops.pop().unwrap();
            with_erase(&uf, &mut self.assoc, op);
        }
        uf.rollback(start);
    }
    fn build(&mut self) -> Vec<Vec<(usize, (usize, usize))>> {
        let mut pend = std::mem::take(&mut self.pend);
        for (&e, &v) in &self.cnt {
            if v >= 1 {
                pend.push(((self.appear[&e], self.q), e));
            }
        }
        let mut segsz = 1;
        while segsz < self.q { segsz <<= 1; }
        let mut seg = vec![vec![]; segsz * 2 - 1];
        for ((l, r), e) in pend {
            self.add(l, r, e, 0, 0, segsz, &mut seg)
        }
        seg
    }
    fn add(
        &self,
        a: usize, b: usize, e: (usize, usize),
        k: usize, l: usize, r: usize,
        segs: &mut [Vec<(usize, (usize, usize))>],
    ) {
        if r <= a || b <= l  { return; }
        if a <= l && r <= b {
            segs[k].push((a, e));
            return;
        }
        self.add(a, b, e, 2 * k + 1, l, (l + r) >> 1, segs);
        self.add(a, b, e, 2 * k + 2, (l + r) >> 1, r, segs);
    }
}

#[derive(Clone, Default)]
struct BinaryMatWithRollback {
    basis: Vec<u32>,
    history: Vec<(u32, u32)>, // (added, sifted)
}

impl BinaryMatWithRollback {
    // O(1)
    fn new() -> Self {
        Default::default()
    }
    // O(rank)
    fn sift(&self, mut x: u32) -> u32 {
        for &b in &self.basis {
            x = std::cmp::min(x, x ^ b);
            if x == 0 {
                return 0;
            }
        }
        x
    }
    // O(rank)
    fn add(&mut self, x: u32) {
        let old = x;
        let x = self.sift(x);
        if x != 0 {
            self.basis.push(x);
            self.history.push((old, x));
        }
    }
    // O(1)
    #[allow(unused)]
    fn rank(&self) -> usize {
        self.basis.len()
    }
    // O(rank)
    #[allow(unused)]
    fn is_indep(&self, x: u32) -> bool {
        self.sift(x) != 0
    }
    // O(1)
    fn rollback(&mut self, x: u32) -> bool {
        if self.history.len() >= 1 {
            if self.history[self.history.len() - 1].0 == x {
                self.history.pop();
                self.basis.pop();
                return true;
            }
        }
        false
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

enum Op {
    Join(usize, usize, BinaryMatWithRollback, BinaryMatWithRollback),
    Add(usize, u32),
}

// Tags: offline-dynamic-connectivity
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize,
        xyd: [(usize1, usize1, u32); m],
        q: usize,
        qs: [switch; q],
    }
    let mut qs = qs;
    qs.reverse();
    for i in (0..m).rev() {
        qs.push(Ok(xyd[i]));
    }
    qs.reverse();
    let q = q + m;

    let mut odc = OfflineDynamicConnectivity::<(BinaryMatWithRollback, u32), Op>::new(n, q);
    for i in 0..q {
        match qs[i] {
            Ok((x, y, _d)) => {
                odc.insert(i, x, y);
            }
            Err((ty, (x, y))) => {
                if ty == 2 {
                    odc.erase(i, x, y);
                }
            }
        }
    }
    odc.run(
        |uf: &PartiallyPersistentUnionFind, k: usize, assoc| {
            match qs[k] {
                Err((3, (mut x, mut y))) => {
                    let mut d = 0;
                    loop {
                        d ^= assoc[x].1;
                        if x == uf.par[x].1 as usize {
                            break;
                        }
                        x = uf.par[x].1 as usize;
                    }
                    loop {
                        d ^= assoc[y].1;
                        if y == uf.par[y].1 as usize {
                            break;
                        }
                        y = uf.par[y].1 as usize;
                    }
                    puts!("{}\n", assoc[x].0.sift(d));
                }
                _ => {}
            }
        },
        |uf: &PartiallyPersistentUnionFind, idx: usize, rx: usize, ry: usize, flipped: bool, assoc: &mut [_]| { // united
            let Ok((mut x, mut y, mut d)) = qs[idx] else { panic!() };
            if flipped {
                std::mem::swap(&mut x, &mut y);
            }
            while x != rx {
                d ^= assoc[x].1;
                x = uf.par[x].1 as usize;
            }
            while y != ry {
                d ^= assoc[y].1;
                y = uf.par[y].1 as usize;
            }
            let op = Op::Join(rx, ry, assoc[rx].0.clone(), assoc[ry].0.clone());
            let tmp = assoc[ry].0.basis.clone();
            assoc[ry] = (BinaryMatWithRollback::new(), d);
            for &v in &tmp {
                assoc[rx].0.add(v);
            }
            op
        },
        |uf: &PartiallyPersistentUnionFind, idx: usize, r: usize, assoc: &mut [_]| { // not united
            let Ok((mut x, mut y, mut d)) = qs[idx] else { panic!() };
            while x != r {
                d ^= assoc[x].1;
                x = uf.par[x].1 as usize;
            }
            while y != r {
                d ^= assoc[y].1;
                y = uf.par[y].1 as usize;
            }
            assoc[r].0.add(d);
            Op::Add(r, d)
        },
        |_uf: &PartiallyPersistentUnionFind, assoc: &mut [_], op| { // erase
            match op {
                Op::Join(rx, ry, matx, maty) => {
                    assoc[rx].0 = matx;
                    assoc[ry] = (maty, 0);
                }
                Op::Add(r, d) => {
                    assoc[r].0.rollback(d);
                }
            }
        },
    )
}
