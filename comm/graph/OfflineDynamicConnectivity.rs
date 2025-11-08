// Ported and modified from:
// - <https://ei1333.github.io/luzhiled/snippets/other/offline-dynamic-connectivity.html>
// Verified by:
// - <https://codeforces.com/contest/938/problem/G> <https://codeforces.com/contest/938/submission/347960635>
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
