use std::io::Read;

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

// Lazy Segment Tree Beats.
// Reference: https://rsm9.hatenablog.com/entry/2021/02/01/220408
pub trait ActionRing {
    type T: Clone + Copy; // data
    type U: Clone + Copy + PartialEq + Eq; // action
    fn biop(x: Self::T, y: Self::T) -> Self::T;
    fn update(x: Self::T, a: Self::U) -> Result<Self::T, ()>;
    fn upop(fst: Self::U, snd: Self::U) -> Self::U;
    fn e() -> Self::T;
    fn upe() -> Self::U; // identity for upop
}
pub struct LazySegTreeBeats<R: ActionRing> {
    n: usize,
    dep: usize,
    dat: Vec<R::T>,
    lazy: Vec<R::U>,
}
impl<R: ActionRing> LazySegTreeBeats<R> {
    pub fn new(n_: usize) -> Self {
        let mut n = 1;
        let mut dep = 0;
        while n < n_ { n *= 2; dep += 1; } // n is a power of 2
        LazySegTreeBeats {
            n: n,
            dep: dep,
            dat: vec![R::e(); 2 * n],
            lazy: vec![R::upe(); n],
        }
    }
    #[allow(unused)]
    pub fn with(a: &[R::T]) -> Self {
        let mut ret = Self::new(a.len());
        let n = ret.n;
        for i in 0..a.len() {
            ret.dat[n + i] = a[i];
        }
        for i in (1..n).rev() {
            ret.update_node(i);
        }
        ret
    }
    #[inline]
    pub fn set(&mut self, idx: usize, x: R::T) {
        debug_assert!(idx < self.n);
        self.apply_any(idx, |_t| x);
    }
    pub fn apply_any<F: Fn(R::T) -> R::T>(&mut self, idx: usize, f: F) {
        debug_assert!(idx < self.n);
        let idx = idx + self.n;
        for i in (1..self.dep + 1).rev() {
            self.push(idx >> i);
        }
        self.dat[idx] = f(self.dat[idx]);
        for i in 1..self.dep + 1 {
            self.update_node(idx >> i);
        }
    }
    pub fn get(&mut self, idx: usize) -> R::T {
        debug_assert!(idx < self.n);
        let idx = idx + self.n;
        for i in (1..self.dep + 1).rev() {
            self.push(idx >> i);
        }
        self.dat[idx]
    }
    /* [l, r) (note: half-inclusive) */
    #[inline]
    pub fn query(&mut self, rng: std::ops::Range<usize>) -> R::T {
        let (l, r) = (rng.start, rng.end);
        debug_assert!(l <= r && r <= self.n);
        if l == r { return R::e(); }
        let mut l = l + self.n;
        let mut r = r + self.n;
        for i in (1..self.dep + 1).rev() {
            if ((l >> i) << i) != l { self.push(l >> i); }
            if ((r >> i) << i) != r { self.push((r - 1) >> i); }
        }
        let mut sml = R::e();
        let mut smr = R::e();
        while l < r {
            if (l & 1) != 0 {
                sml = R::biop(sml, self.dat[l]);
                l += 1;
            }
            if (r & 1) != 0 {
                r -= 1;
                smr = R::biop(self.dat[r], smr);
            }
            l >>= 1;
            r >>= 1;
        }
        R::biop(sml, smr)
    }
    /* ary[i] = upop(ary[i], v) for i in [l, r) (half-inclusive) */
    #[inline]
    pub fn update(&mut self, rng: std::ops::Range<usize>, f: R::U)  {
        let (l, r) = (rng.start, rng.end);
        debug_assert!(l <= r && r <= self.n);
        if l == r { return; }
        let mut l = l + self.n;
        let mut r = r + self.n;
        for i in (1..self.dep + 1).rev() {
            if ((l >> i) << i) != l { self.push(l >> i); }
            if ((r >> i) << i) != r { self.push((r - 1) >> i); }
        }
        {
            let l2 = l;
            let r2 = r;
            while l < r {
                if (l & 1) != 0 {
                    self.all_apply(l, f);
                    l += 1;
                }
                if (r & 1) != 0 {
                    r -= 1;
                    self.all_apply(r, f);
                }
                l >>= 1;
                r >>= 1;
            }
            l = l2;
            r = r2;
        }
        for i in 1..self.dep + 1 {
            if ((l >> i) << i) != l { self.update_node(l >> i); }
            if ((r >> i) << i) != r { self.update_node((r - 1) >> i); }
        }
    }
    #[inline]
    fn update_node(&mut self, k: usize) {
        self.dat[k] = R::biop(self.dat[2 * k], self.dat[2 * k + 1]);
    }
    fn all_apply(&mut self, k: usize, f: R::U) {
        let maybe_dat = R::update(self.dat[k], f);
        let fail = maybe_dat.is_err();
        if let Ok(dat) = maybe_dat {
            self.dat[k] = dat;
        }
        if k < self.n {
            self.lazy[k] = R::upop(self.lazy[k], f);
            if fail {
                self.push(k);
                self.update_node(k);
            }
        }
    }
    fn push(&mut self, k: usize) {
        let val = self.lazy[k];
        self.all_apply(2 * k, val);
        self.all_apply(2 * k + 1, val);
        self.lazy[k] = R::upe();
    }
}

enum CSAR70AOM {}

type Int = u32; // Change here to change type
const FULL: Int = (1 << 20) - 1;
impl ActionRing for CSAR70AOM {
    type T = (Int, Int, Int); // and, or, max
    type U = (Int, Int); // (a, b): x |-> (x & a) | b
    fn biop((and1, or1, max1): Self::T, (and2, or2, max2): Self::T) -> Self::T {
        (and1 & and2, or1 | or2, max1.max(max2))
    }
    // Complexity note: potential = (or ^ and).count_ones()
    fn update((and, or, max): Self::T, up: Self::U) -> Result<Self::T, ()> {
        let relevant = (up.0 ^ FULL) | up.1;
        if relevant == 0 {
            // NOP
            return Ok((and, or, max));
        }
        if (and & relevant) == (or & relevant) {
            return Ok(((and & up.0) | up.1, (or & up.0) | up.1, (max & up.0) | up.1));
        }
        Err(())
    }
    fn upop((fst1, fst2): Self::U, (snd1, snd2): Self::U) -> Self::U {
        (fst1 & snd1, (fst2 & snd1) | snd2)
    }
    fn e() -> Self::T {
        (FULL, 0, 0)
    }
    fn upe() -> Self::U { // identity for upop
        (FULL, 0)
    }
}

// The author read the solution before implementing this.
// Tags: segment-tree-beats
// https://rsm9.hatenablog.com/entry/2021/02/01/220408
fn main() {
    let n: usize = get();
    let q: usize = get();
    let mut st = LazySegTreeBeats::<CSAR70AOM>::new(n);
    for i in 0..n {
        let x: Int = get();
        st.set(i, (x, x, x));
    }
    for _ in 0..q {
        let ty: i32 = get();
        let l = get::<usize>() - 1;
        let r = get::<usize>();
        if ty != 3 {
            let x: Int = get();
            let op = if ty == 1 {
                (x, 0)
            } else {
                (FULL, x)
            };
            st.update(l..r, op);
        } else {
            let res = st.query(l..r);
            println!("{}", res.2);
        }
    }
}
