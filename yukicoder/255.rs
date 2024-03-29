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

enum VFix {}

const B: usize = 2;

impl ActionRing for VFix {
    type T = [i64; B]; // data
    type U = [i64; B]; // action, [[a, 0], [b, 1]]
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
            ans[0] += x[i] * o[i];
        }
        ans[1] = x[1];
        ans
    }
    fn upop(fst: Self::U, snd: Self::U) -> Self::U {
        let mut ans = [0.into(); B];
        for i in 0..B {
            ans[i] += fst[i] * snd[0];
        }
        ans[1] += snd[1];
        ans
    }
    fn e() -> Self::T {
        [0.into(); B]
    }
    fn upe() -> Self::U { // identity for upop
        [1, 0]
    }
}

const MOD: i64 = 1_000_000_000_000_000_009;

fn main() {
    input! {
        n: i64,
        q: usize,
        xlr: [(i32, i64, i64); q],
    }
    let mut coo = vec![0, n];
    for &(_, l, r) in &xlr {
        coo.push(l);
        coo.push(r + 1);
    }
    coo.sort(); coo.dedup();
    let m = coo.len() - 1;
    let mut st = vec![];
    let mut a = vec![[0; B]; m];
    for i in 0..m {
        a[i][1] = coo[i + 1] - coo[i];
    }
    for _ in 0..5 {
        st.push(LazySegTree::<VFix>::with(&a));
    }
    let mut sc = [0; 5];
    for (x, l, r) in xlr {
        let l = coo.binary_search(&l).unwrap();
        let r = coo.binary_search(&(r + 1)).unwrap();
        if x == 0 {
            let mut val = [(0, 0); 5];
            for i in 0..5 {
                val[i] = (st[i].query(l, r)[0], i);
            }
            val.sort(); val.reverse();
            if val[0].0 > val[1].0 {
                sc[val[0].1] += val[0].0;
                sc[val[0].1] %= MOD;
            }
        } else {
            let idx = x as usize - 1;
            for i in 0..5 {
                if i == idx {
                    st[i].update(l, r, [1, 1]);
                } else {
                    st[i].update(l, r, [0, 0]);
                }
            }
        }
    }
    for i in 0..5 {
        sc[i] += st[i].query(0, m)[0];
        sc[i] %= MOD;
    }
    println!("{} {} {} {} {}", sc[0], sc[1], sc[2], sc[3], sc[4]);
}
