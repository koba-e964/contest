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
    fn update(x: Self::T, a: Self::U) -> (Self::T, bool /* fail */);
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
        let (dat, fail) = R::update(self.dat[k], f);
        self.dat[k] = dat;
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

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

enum Affine {}

type AffineInt = i64; // Change here to change type
const INF: i64 = 1 << 40;
impl ActionRing for Affine {
    type T = (AffineInt, AffineInt, AffineInt, AffineInt); // data, size, max, lcm
    type U = Result<AffineInt, AffineInt>; // action, Ok(g): x |-> gcd(x, g), Err(v): _x |-> v
    fn biop((x, s, ma1, lcm1): Self::T, (y, t, ma2, lcm2): Self::T) -> Self::T {
        let l = if lcm1 >= INF || lcm2 >= INF {
            INF
        } else {
            let g = gcd(lcm1, lcm2);
            std::cmp::min((lcm1 / g).saturating_mul(lcm2), INF)
        };
        (x + y, s + t, std::cmp::max(ma1, ma2), l)
    }
    // Complexity note: potential = ma * s == x ? 0 : sum of num of factors(values)
    // (ma * s == x is necessary because without this condition one range_assign can increase potential by N log_2(val))
    // If update fails, potential decreases by >= 1.
    fn update((x, s, ma, lcm): Self::T, up: Self::U) -> (Self::T, bool) {
        let g = match up {
            Ok(g) => g,
            Err(v) => return ((v * s, s, v, v), false),
        };
        if g == 0 {
            return ((x, s, ma, lcm), false);
        }
        if x == s * ma {
            // All elements are equal. Cannot fail.
            let newval = gcd(ma, g);
            return ((newval * s, s, newval, newval), false);
        }
        if lcm < INF && g % lcm == 0 {
            // NOP
            return ((x, s, ma, lcm), false);
        }
        ((x, s, ma, lcm), true)
    }
    fn upop(fst: Self::U, snd: Self::U) -> Self::U {
        let g2 = match snd {
            Ok(g) => g,
            Err(_) => return snd,
        };
        match fst {
            Ok(g) => Ok(gcd(g, g2)),
            Err(v) => Err(gcd(g2, v)),
        }
    }
    fn e() -> Self::T {
        (0.into(), 0.into(), 0.into(), 1.into())
    }
    fn upe() -> Self::U { // identity for upop
        Ok(0)
    }
}

// Tags: segment-tree-beats
fn main() {
    let n: usize = get();
    let q: usize = get();
    let a: Vec<_> = (0..n).map(|_| {
        let x: i64 = get();
        (x, 1, x, x)
    }).collect();
    let mut st = LazySegTreeBeats::<Affine>::with(&a);
    for _ in 0..q {
        let ty: i32 = get();
        let l = get::<usize>() - 1;
        let r: usize = get();
        if ty == 1 || ty == 2 {
            let x: i64 = get();
            if ty == 1 {
                st.update(l..r, Err(x));
            } else {
                st.update(l..r, Ok(x));
            }
        } else {
            let val = st.query(l..r);
            println!("{}", if ty == 3 { val.2 } else { val.0 });
        }
    }
}
