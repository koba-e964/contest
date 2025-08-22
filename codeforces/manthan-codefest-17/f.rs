fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

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

enum CFMC17FNagini {}

type Int = i64; // Change here to change type
const INF: Int = 1 << 30;

mod beats_chmin {
    use super::Int;
    pub fn max((a_max, a_smax, a_c): (Int, Int, Int), (b_max, b_smax, b_c): (Int, Int, Int)) -> (Int, Int, Int) {
        if a_max < b_max {
            (b_max, b_smax.max(a_max), b_c)
        } else if a_max > b_max {
            (a_max, a_smax.max(b_max), a_c)
        } else {
            (a_max, a_smax.max(b_smax), a_c + b_c)
        }
    }

    pub fn chmin((a_max, a_smax, a_c): (Int, Int, Int), x: Int) -> Result<(Int, Int, Int), ()> {
        if a_max <= x {
            return Ok((a_max, a_smax, a_c));
        }
        if a_smax < x {
            return Ok((x, a_smax, a_c));
        }
        Err(())
    }
}

impl ActionRing for CFMC17FNagini {
    type T = ((Int, Int, Int), (Int, Int, Int), Int); // max1, smax1, c1, max2, smax2, c2, sum
    type U = (Int, Int); // (a, b): (x, y) |-> (x.min(a), y.min(b))
    fn biop((a1, a2, a_sum): Self::T, (b1, b2, b_sum): Self::T) -> Self::T {
        let s1 = beats_chmin::max(a1, b1);
        let s2 = beats_chmin::max(a2, b2);
        (s1, s2, a_sum + b_sum)
    }
    // Complexity note: potential = 2 * num - a_c1 - a_c2 + (a_max1 == INF ? 1 : 0) + (a_max2 == INF ? 1 : 0)
    fn update(((a_max1, a_smax1, a_c1), (a_max2, a_smax2, a_c2), a_sum): Self::T, (up1, up2): Self::U) -> Result<Self::T, ()> {
        if up1 == INF && up2 == INF {
            return Ok(((a_max1, a_smax1, a_c1), (a_max2, a_smax2, a_c2), a_sum));
        }
        let ma_inf = a_max1 == INF || a_max2 == INF;
        let sma_ninf = a_smax1 == -INF && a_smax2 == -INF;
        if ma_inf && !sma_ninf {
            return Err(());
        }
        let s1 = beats_chmin::chmin((a_max1, a_smax1, a_c1), up1)?;
        let s2 = beats_chmin::chmin((a_max2, a_smax2, a_c2), up2)?;
        if ma_inf {
            let val = if s1.0.max(s2.0) == INF {
                0
            } else {
                s1.0 * s1.2 + s2.0 * s2.2
            };
            return Ok((s1, s2, val));
        }
        let mut s_sum = a_sum;
        s_sum += (s1.0 - a_max1) * s1.2;
        s_sum += (s2.0 - a_max2) * s2.2;
        Ok((s1, s2, s_sum))
    }
    fn upop((fst1, fst2): Self::U, (snd1, snd2): Self::U) -> Self::U {
        (fst1.min(snd1), fst2.min(snd2))
    }
    fn e() -> Self::T {
        let i = (-INF, -INF, 0);
        (i, i, 0)
    }
    fn upe() -> Self::U { // identity for upop
        (INF, INF)
    }
}

// The author read the solution before implementing this.
// Tags: segment-tree-beats
// https://rsm9.hatenablog.com/entry/2021/02/01/220408
// https://smijake3.hatenablog.com/entry/2019/04/28/021457
// メモ: st.set で無限ループ。作用が (INF, INF) の時にスキップすることで回避。
// メモ: 両方とも <INF のときにだけ sum をとるのがバグっていたが、直した。
// メモ: どちらか一方が INF のとき sum は正しく計算できないことが多い。
// 最大が INF で最小が -INF のときだけ正しく計算できる。
fn main() {
    let q: i32 = getline().trim().parse().unwrap();
    const W: usize = 100_000;
    let init = vec![((INF, -INF, 1), (INF, -INF, 1), 0); W];
    let mut st = LazySegTreeBeats::<CFMC17FNagini>::with(&init);
    for _ in 0..q {
        let ints = getline().trim().split_whitespace()
            .map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();
        let ty: i32 = ints[0];
        let l = ints[1] as usize;
        let r = ints[2] as usize;
        if ty == 1 {
            let k: i64 = ints[3] as i64;
            let op = if k > 0 {
                (k, INF)
            } else {
                (INF, -k)
            };
            st.update(l..r, op);
        } else {
            let val = st.query(l..r);
            println!("{}", val.2);
        }
    }
}
