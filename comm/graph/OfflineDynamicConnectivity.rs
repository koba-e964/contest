// Ported and modified from:
// - <https://ei1333.github.io/luzhiled/snippets/other/offline-dynamic-connectivity.html>
// Depends on: graph/PersistentUnionFind.rs
// Verified by:
// - <https://codeforces.com/contest/938/problem/G> <https://codeforces.com/contest/938/submission/347960635>
struct OfflineDynamicConnectivity<T, Op> {
    n: usize,
    q: usize,
    pend: Vec<((u32, u32), E)>,
    cnt: std::collections::HashMap<E, u32>,
    appear: std::collections::HashMap<E, u32>,
    assoc: Vec<T>,
    ops: Vec<Op>,
}
type E = (u32, u32);

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
        let e = (s.min(t) as u32, s.max(t) as u32);
        let old = self.cnt.get(&e).cloned().unwrap_or(0);
        self.cnt.insert(e, old + 1);
        if old == 0 {
            self.appear.insert(e, idx as _);
        }
        self.q = self.q.max(idx + 1);
    }
    pub fn erase(&mut self, idx: usize, s: usize, t: usize) {
        let e = (s.min(t) as u32, s.max(t) as u32);
        let old = self.cnt[&e];
        assert!(old >= 1);
        self.cnt.insert(e, old - 1);
        if old == 1 {
            let ap = self.appear[&e];
            self.pend.push(((ap, idx as u32), e));
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
        k: u32,
        uf: &mut PartiallyPersistentUnionFind,
        seg: &[Vec<(u32, E)>],
    ) {
        let start = uf.now();
        for &(idx, (x, y)) in &seg[k as usize] {
            let now = uf.now();
            let rx = uf.find(x as _, now);
            let ry = uf.find(y as _, now);
            let united = uf.unite(x as _, y as _);
            let r = uf.find(rx, uf.now());
            let flipped = r == ry;
            let op = if united {
                with_unite(&uf, idx as _, r as _, (rx + ry - r) as _, flipped, &mut self.assoc)
            } else {
                with_same(&uf, idx as _, r as _, &mut self.assoc)
            };
            self.ops.push(op);
        }
        let segsz = (seg.len() as u32 + 1) / 2;
        if k < segsz - 1 {
            self.run_inner(f, with_unite, with_same, with_erase, 2 * k + 1, uf, seg);
            self.run_inner(f, with_unite, with_same, with_erase, 2 * k + 2, uf, seg);
        } else if k - (segsz - 1) < self.q as u32 {
            let query_index = k - (segsz - 1);
            f(&uf, query_index as _, &self.assoc);
        }
        for _ in seg[k as usize].iter().rev() {
            let op = self.ops.pop().unwrap();
            with_erase(&uf, &mut self.assoc, op);
        }
        uf.rollback(start);
    }
    fn build(&mut self) -> Vec<Vec<(u32, E)>> {
        let mut pend = std::mem::take(&mut self.pend);
        for (&e, &v) in &self.cnt {
            if v >= 1 {
                pend.push(((self.appear[&e], self.q as u32), e));
            }
        }
        let mut segsz = 1;
        while segsz < self.q { segsz <<= 1; }
        let mut seg = vec![vec![]; segsz * 2 - 1];
        for ((l, r), e) in pend {
            self.add(l, r, e, 0, 0, segsz as u32, &mut seg)
        }
        seg
    }
    fn add(
        &self,
        a: u32, b: u32, e: E,
        k: u32, l: u32, r: u32,
        seg: &mut [Vec<(u32, E)>],
    ) {
        if r <= a || b <= l  { return; }
        if a <= l && r <= b {
            seg[k as usize].push((a, e));
            return;
        }
        self.add(a, b, e, 2 * k + 1, l, (l + r) >> 1, seg);
        self.add(a, b, e, 2 * k + 2, (l + r) >> 1, r, seg);
    }
}
