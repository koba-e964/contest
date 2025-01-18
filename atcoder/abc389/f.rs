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

// Reference: https://github.com/atcoder/ac-library/blob/master/atcoder/lazysegtree.hpp
// Verified by: https://atcoder.jp/contests/joisc2021/submissions/27733144
pub trait Action {
    type T: Clone + Copy; // data
    type U: Clone + Copy + PartialEq + Eq; // action
    fn update(x: Self::T, a: Self::U) -> Self::T;
    fn upop(fst: Self::U, snd: Self::U) -> Self::U;
    fn upe() -> Self::U; // identity for upop
}
pub struct DualSegTree<R: Action> {
    n: usize,
    dat: Vec<R::T>,
    lazy: Vec<R::U>,
    dep: usize,
}

impl<R: Action> DualSegTree<R> {
    pub fn new(a: &[R::T]) -> Self {
        let n_ = a.len();
        let mut n = 1;
        let mut dep = 0;
        while n < n_ { n *= 2; dep += 1; } // n is a power of 2
        let mut a = a.to_vec();
        let filler = a[0];
        a.resize(n, filler);
        DualSegTree {
            n: n,
            dat: a,
            lazy: vec![R::upe(); n],
            dep: dep,
        }
    }
    #[inline]
    pub fn set(&mut self, idx: usize, x: R::T) {
        debug_assert!(idx < self.n);
        self.apply_any(idx, |_t| x);
    }
    #[inline]
    pub fn apply(&mut self, idx: usize, f: R::U) {
        debug_assert!(idx < self.n);
        self.apply_any(idx, |t| R::update(t, f));
    }
    pub fn apply_any<F: Fn(R::T) -> R::T>(&mut self, idx: usize, f: F) {
        debug_assert!(idx < self.n);
        let idx = idx + self.n;
        for i in (1..self.dep + 1).rev() {
            self.push(idx >> i);
        }
        let to = &mut self.dat[idx - self.n];
        *to = f(*to);
    }
    pub fn get(&mut self, idx: usize) -> R::T {
        debug_assert!(idx < self.n);
        let idx = idx + self.n;
        for i in (1..self.dep + 1).rev() {
            self.push(idx >> i);
        }
        self.dat[idx - self.n]
    }
    /* ary[i] = upop(ary[i], v) for i in [l, r) (half-inclusive) */
    #[inline]
    pub fn update(&mut self, l: usize, r: usize, f: R::U)  {
        debug_assert!(l <= r && r <= self.n);
        if l == r { return; }
        let mut l = l + self.n;
        let mut r = r + self.n;
        for i in (1..self.dep + 1).rev() {
            if ((l >> i) << i) != l { self.push(l >> i); }
            if ((r >> i) << i) != r { self.push((r - 1) >> i); }
        }
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
    }
    fn all_apply(&mut self, k: usize, f: R::U) {
        if k >= self.n {
            self.dat[k - self.n] = R::update(self.dat[k - self.n], f);
        }
        if k < self.n {
            self.lazy[k] = R::upop(self.lazy[k], f);
        }
    }
    fn push(&mut self, k: usize) {
        let val = self.lazy[k];
        self.all_apply(2 * k, val);
        self.all_apply(2 * k + 1, val);
        self.lazy[k] = R::upe();
    }
}

enum Add {}
type AddInt = i32;

impl Action for Add {
    type T = AddInt; // data
    type U = AddInt;
    fn update(x: Self::T, a: Self::U) -> Self::T {
        x + a
    }
    fn upop(fst: Self::U, snd: Self::U) -> Self::U {
        fst + snd
    }
    fn upe() -> Self::U { // identity for upop
        0
    }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        lr: [(usize, usize); n],
        q: usize,
        x: [usize; q],
    }
    const W: usize = 500_001;
    let mut init = vec![0; W];
    for i in 0..W {
        init[i] = i as i32;
    }
    let mut st = DualSegTree::<Add>::new(&init);
    let lower_bound = |st: &mut DualSegTree<Add>, x: i32| {
        let mut pass = W + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if st.get(mid - 1) >= x {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    };
    for &(l, r) in &lr {
        let idx1 = lower_bound(&mut st, l as i32);
        let idx2 = lower_bound(&mut st, r as i32 + 1);
        st.update(idx1, idx2, 1);
    }
    for x in x {
        puts!("{}\n", st.get(x));
    }
}
