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

/**
 * Lazy Segment Tree. This data structure is useful for fast folding and updating on intervals of an array
 * whose elements are elements of monoid T. Note that constructing this tree requires the identity
 * element of T and the operation of T. This is monomorphised, because of efficiency. T := i64, biop = max, upop = (+)
 * Reference: http://d.hatena.ne.jp/kyuridenamida/20121114/1352835261
 * Verified by https://codeforces.com/contest/1114/submission/49759034
 */
pub trait ActionRing {
    type T: Clone + Copy; // data
    type U: Clone + Copy + PartialEq + Eq; // action
    fn biop(x: Self::T, y: Self::T) -> Self::T;
    fn update(x: Self::T, a: Self::U, height: usize) -> Self::T;
    fn upop(fst: Self::U, snd: Self::U) -> Self::U;
    fn e() -> Self::T;
    fn upe() -> Self::U; // identity for upop
}
pub struct LazySegTree<R: ActionRing> {
    n: usize,
    dep: usize,
    dat: Vec<R::T>,
    lazy: Vec<R::U>,
}

impl<R: ActionRing> LazySegTree<R> {
    #[allow(unused)]
    pub fn new(n_: usize) -> Self {
        let mut n = 1;
        let mut dep = 0;
        while n < n_ { n *= 2; dep += 1; } // n is a power of 2
        LazySegTree {
            n: n,
            dep: dep,
            dat: vec![R::e(); 2 * n - 1],
            lazy: vec![R::upe(); 2 * n - 1]
        }
    }
    #[allow(unused)]
    pub fn with(a: &[R::T]) -> Self {
        let n_ = a.len();
        let mut n = 1;
        let mut dep = 0;
        while n < n_ { n *= 2; dep += 1; } // n is a power of 2
        let mut dat = vec![R::e(); 2 * n - 1];
        for i in 0..n_ {
            dat[n - 1 + i] = a[i];
        }
        for i in (0..n - 1).rev() {
            dat[i] = R::biop(dat[2 * i + 1], dat[2 * i + 2]);
        }
        LazySegTree {
            n: n,
            dep: dep,
            dat: dat,
            lazy: vec![R::upe(); 2 * n - 1],
        }
    }
    #[inline]
    fn lazy_evaluate_node(&mut self, k: usize, height: usize) {
        if self.lazy[k] == R::upe() { return; }
        self.dat[k] = R::update(self.dat[k], self.lazy[k], height);
        if k < self.n - 1 {
            self.lazy[2 * k + 1] = R::upop(self.lazy[2 * k + 1], self.lazy[k]);
            self.lazy[2 * k + 2] = R::upop(self.lazy[2 * k + 2], self.lazy[k]);
        }
        self.lazy[k] = R::upe(); // identity for upop
    }
    #[inline]
    fn update_node(&mut self, k: usize) {
        self.dat[k] = R::biop(self.dat[2 * k + 1], self.dat[2 * k + 2]);
    }
    fn update_sub(&mut self, a: usize, b: usize, v: R::U, k: usize, height: usize, l: usize, r: usize) {
        self.lazy_evaluate_node(k, height);

        // [a,b) and  [l,r) intersects?
        if r <= a || b <= l {return;}
        if a <= l && r <= b {
            self.lazy[k] = R::upop(self.lazy[k], v);
            self.lazy_evaluate_node(k, height);
            return;
        }

        self.update_sub(a, b, v, 2 * k + 1, height - 1, l, (l + r) / 2);
        self.update_sub(a, b, v, 2 * k + 2, height - 1, (l + r) / 2, r);
        self.update_node(k);
    }
    /* ary[i] = upop(ary[i], v) for i in [a, b) (half-inclusive) */
    #[inline]
    pub fn update(&mut self, a: usize, b: usize, v: R::U) {
        let n = self.n;
        let dep = self.dep;
        self.update_sub(a, b, v, 0, dep, 0, n);
    }
    /* l,r are for simplicity */
    fn query_sub(&mut self, a: usize, b: usize, k: usize, height: usize, l: usize, r: usize) -> R::T {
        self.lazy_evaluate_node(k, height);

        // [a,b) and  [l,r) intersect?
        if r <= a || b <= l {return R::e();}
        if a <= l && r <= b {return self.dat[k];}
        let vl = self.query_sub(a, b, 2 * k + 1, height - 1, l, (l + r) / 2);
        let vr = self.query_sub(a, b, 2 * k + 2, height - 1, (l + r) / 2, r);
        self.update_node(k);
        R::biop(vl, vr)
    }
    /* [a, b) (note: half-inclusive) */
    #[inline]
    pub fn query(&mut self, a: usize, b: usize) -> R::T {
        let n = self.n;
        let dep = self.dep;
        self.query_sub(a, b, 0, dep, 0, n)
    }
}

enum V {}

const B: usize = 3;

impl ActionRing for V {
    type T = [i64; B]; // data
    type U = [[i64; B]; B]; // action, (a, b) |-> x |-> ax + b
    fn biop(x: Self::T, y: Self::T) -> Self::T {
        let mut ans = [0.into(); B];
        for i in 0..B {
            ans[i] = x[i] + y[i];
        }
        ans
    }
    fn update(x: Self::T, o: Self::U, _height: usize) -> Self::T {
        let mut ans = [0.into(); B];
        for i in 0..B {
            for j in 0..B {
                ans[j] += x[i] * o[i][j];
            }
        }
        ans
    }
    fn upop(fst: Self::U, snd: Self::U) -> Self::U {
        let mut ans = [[0.into(); B]; B];
        for i in 0..B {
            for j in 0..B {
                for k in 0..B {
                    ans[i][k] += fst[i][j] * snd[j][k];
                }
            }
        }
        ans
    }
    fn e() -> Self::T {
        [0.into(); B]
    }
    fn upe() -> Self::U { // identity for upop
        let mut ans = [[0.into(); B]; B];
        for i in 0..B {
            ans[i][i] = 1.into();
        }
        ans
    }
}

// Tags: lazy-segment-trees, linear-transformations
fn main() {
    let n: usize = get();
    let q: usize = get();
    let a: Vec<i64> = (0..n).map(|_| get()).collect();
    let mut b = vec![[0, 0, 1]; n];
    for i in 0..n {
        b[i][1] = a[i] / 2 * 2;
        if a[i] % 2 == 0 {
            b[i][0] = -1;
        } else {
            b[i][0] = 1;
        }
    }
    let mut st = LazySegTree::<V>::with(&b);
    for _ in 0..q {
        let ty: i32 = get();
        let l = get::<usize>() - 1;
        let r: usize = get();
        if ty == 3 {
            let s = st.query(l, r);
            println!("{}", s[1] + (s[2] + s[0]) / 2);
        } else if ty == 2 {
            let x: i64 = get();
            let mut tr = [[0; B]; B];
            for i in 0..B {
                tr[i][i] = 1;
            }
            tr[2][1] = x / 2 * 2;
            st.update(l, r, tr);
            if x % 2 == 1 {
                let mut tr = [[0; B]; B];
                for i in 0..B {
                    tr[i][i] = 1;
                }
                tr[0][0] = -1;
                tr[2][1] = 1;
                tr[0][1] = 1;
                st.update(l, r, tr);
            }
        } else {
            let mut tr = [[0; B]; B];
            tr[0][0] = 1;
            tr[2][2] = 1;
            st.update(l, r, tr);
        }
    }
}
