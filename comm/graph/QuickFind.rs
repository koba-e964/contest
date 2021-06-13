// Quick-Find data structure.
// Verified by: https://atcoder.jp/contests/cf17-tournament-round3-open/submissions/22928265
// Verified by: https://atcoder.jp/contests/ttpc2019/submissions/23384553 (polymorphic version)
struct QuickFind<T = ()> {
    root: Vec<usize>, mem: Vec<Vec<usize>>,
    dat: Vec<T>, default: T,
}

impl QuickFind<()> {
    #[allow(unused)]
    fn new(n: usize) -> Self {
        Self::with_dat(n, ())
    }
    #[allow(unused)]
    fn unite(&mut self, x: usize, y: usize) {
        self.unite_with_hooks(x, y, |&(), _| (), |(), ()| ());
    }
}
impl<T: Clone> QuickFind<T> {
    fn with_dat(n: usize, def: T) -> Self {
        let root = (0..n).collect();
        let mut mem = vec![vec![]; n];
        for i in 0..n {
            mem[i] = vec![i];
        }
        QuickFind { root: root, mem: mem, dat: vec![def.clone(); n], default: def }
    }
    fn root(&self, x: usize) -> usize {
        self.root[x]
    }
    #[allow(unused)]
    fn set(&mut self, idx: usize, val: T) {
        let r = self.root[idx];
        self.dat[r] = val;
    }
    // unite always merges y to x if |x| >= |y|.
    fn unite_with_hooks<F: FnMut(&T, i64), G: FnMut(T, T) -> T>(
        &mut self, x: usize, y: usize,
        mut hook: F, mut merge: G) {
        let mut x = self.root(x);
        let mut y = self.root(y);
        if x == y { return }
        if self.mem[x].len() < self.mem[y].len() {
            std::mem::swap(&mut x, &mut y);
        }
        let memy = std::mem::replace(&mut self.mem[y], vec![]);
        for &v in &memy {
            self.root[v] = x;
        }
        self.mem[x].extend_from_slice(&memy);
        // hook
        hook(&self.dat[x], -1);
        hook(&self.dat[y], -1);
        self.dat[x] = merge(
            std::mem::replace(&mut self.dat[x], self.default.clone()),
            std::mem::replace(&mut self.dat[y], self.default.clone()),
        );
        hook(&self.dat[x], 1);
    }
    #[allow(unused)]
    fn is_same_set(&self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    #[allow(unused)]
    fn size(&self, x: usize) -> usize {
        let x = self.root(x);
        self.mem[x].len()
    }
}
