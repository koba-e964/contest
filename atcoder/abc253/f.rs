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

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

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
type AddInt = i64;

impl Action for Add {
    type T = AddInt; // data
    type U = AddInt; // action, b |-> x |-> x + b
    fn update(x: Self::T, b: Self::U) -> Self::T {
        x + b
    }
    fn upop(fst: Self::U, snd: Self::U) -> Self::U {
        fst + snd
    }
    fn upe() -> Self::U { // identity for upop
        0
    }
}

fn main() {
    let n: usize = get();
    let m: usize = get();
    let q: usize = get();
    let mut rows = vec![(0, 0); n];
    let mut ans = vec![];
    let mut que = vec![];
    for i in 0..q {
        let ty: i32 = get();
        if ty == 1 {
            let l: usize = get();
            let r: usize = get();
            let x: i64 = get();
            que.push((i + 1, l - 1, r, x));
        } else if ty == 2 {
            let x: usize = get();
            let y: i64 = get();
            rows[x - 1] = (i + 1, y);
        } else {
            let x: usize = get();
            let y: usize = get();
            let idx = ans.len();
            let now = rows[x - 1];
            ans.push(now.1);
            que.push((now.0, y - 1, idx, -1));
            que.push((i + 1, y - 1, idx, -2));
        }
    }
    que.sort();
    let mut st = DualSegTree::<Add>::new(&vec![0; m]);
    for (_, l, r, x) in que {
        if x >= 0 {
            st.update(l, r, x);
        } else {
            let val = st.get(l);
            ans[r] += val * if x == -1 { -1 } else { 1 };
        }
    }
    for ans in ans {
        println!("{}", ans);
    }
}
