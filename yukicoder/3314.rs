fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
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

type Int = i64; // Change here to change type
const INF: Int = 1 << 60;

mod beats_chmax_sum {
    use super::Int;
    pub fn op((a_min, a_smin, a_c, a_sum): (Int, Int, Int, Int), (b_min, b_smin, b_c, b_sum): (Int, Int, Int, Int)) -> (Int, Int, Int, Int) {
        if a_min > b_min {
            (b_min, b_smin.min(a_min), b_c, a_sum + b_sum)
        } else if a_min < b_min {
            (a_min, a_smin.min(b_min), a_c, a_sum + b_sum)
        } else {
            (a_min, a_smin.min(b_smin), a_c + b_c, a_sum + b_sum)
        }
    }

    pub fn chmax((a_min, a_smin, a_c, a_sum): (Int, Int, Int, Int), x: Int) -> Result<(Int, Int, Int, Int), ()> {
        if a_min >= x {
            return Ok((a_min, a_smin, a_c, a_sum));
        }
        if a_smin > x {
            return Ok((x, a_smin, a_c, a_sum + (x - a_min) * a_c));
        }
        Err(())
    }
}

enum Yuki3314LR {}

impl ActionRing for Yuki3314LR {
    type T = (Int, Int, Int, Int); // min, smin, c, sum
    type U = Int; // a: x |-> max(x, a)
    fn biop(a: Self::T, b: Self::T) -> Self::T {
        beats_chmax_sum::op(a, b)
    }
    // Complexity note: potential = (or ^ and).count_ones()
    fn update(a: Self::T, up: Self::U) -> Result<Self::T, ()> {
        if up == -INF {
            return Ok(a);
        }
        beats_chmax_sum::chmax(a, up)
    }
    fn upop(fst: Self::U, snd: Self::U) -> Self::U {
        fst.max(snd)
    }
    fn e() -> Self::T {
        (INF, INF, 0, 0)
    }
    fn upe() -> Self::U { // identity for upop
        -INF
    }
}

fn parallel_binary_search_once(
    a: &[i64],
    upd: &[(usize, usize, i64)],
    qs: &[(usize, usize, i64)],
    mid: &[usize],
) -> Vec<bool> {
    let k = upd.len();
    let n = a.len();
    let q = qs.len();
    let mut init = vec![(0, 0, 0, 0); n];
    for i in 0..n {
        init[i] = (a[i], INF, 1, a[i]);
    }
    let mut st = LazySegTreeBeats::<Yuki3314LR>::with(&init);
    let mut ev = vec![vec![]; k + 1];
    for i in 0..q {
        ev[k.min(mid[i])].push(i);
    }
    let mut ans = vec![false; q];
    for t in 0..k + 1 {
        for &idx in &ev[t] {
            let (l, r, x) = qs[idx];
            ans[idx] = st.query(l..r).3 >= x;
        }
        if t < k {
            let (l, r, x) = upd[t];
            st.update(l..r, x);
        }
    }
    ans
}

// https://yukicoder.me/problems/no/3314 (3.5)
// Tags: segment-tree-beats, parallel-binary-search
fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let [_n, k, q] = ints[..] else { panic!() };
    let a = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let upd = (0..k).map(|_| {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        (ints[0] as usize - 1, ints[1] as usize, ints[2])
    }).collect::<Vec<_>>();
    let qs = (0..q).map(|_| {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        (ints[0] as usize - 1, ints[1] as usize, ints[2])
    }).collect::<Vec<_>>();
    let mut pass = vec![(1 << 14) - 1; q];
    let mut fail = vec![-1; q];
    for _ in 0..14 {
        let mut mid = vec![0; q];
        for i in 0..q {
            mid[i] = ((pass[i] + fail[i]) / 2) as usize;
        }
        let res = parallel_binary_search_once(&a, &upd, &qs, &mid);
        for i in 0..q {
            if res[i] {
                pass[i] = mid[i] as i32;
            } else {
                fail[i] = mid[i] as i32;
            }
        }
    }
    for a in pass {
        println!("{}", if a > k as i32 {
            -1
        } else {
            a
        });
    }
}
