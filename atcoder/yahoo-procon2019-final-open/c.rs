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

enum Affine {}

impl ActionRing for Affine {
    type T = i64; // data
    type U = (i64, i64); // action, (a, b) |-> x |-> ax + b
    fn biop(x: Self::T, y: Self::T) -> Self::T {
        x + y
    }
    fn update(x: Self::T, (a, b): Self::U, height: usize) -> Self::T {
        x * a + (b << height)
    }
    fn upop(fst: Self::U, snd: Self::U) -> Self::U {
        let (a, b) = fst;
        let (c, d) = snd;
        (a * c, b * c + d)
    }
    fn e() -> Self::T {
        0
    }
    fn upe() -> Self::U { // identity for upop
        (1, 0)
    }
}

fn calc(a: &[(i64, i64, i64, i64)]) -> i64 {
    let mut coord = vec![];
    for &(_c, d, _e, f) in a {
        coord.push(d);
        coord.push(f);
    }
    coord.sort_unstable(); coord.dedup();
    let m = coord.len();
    if m <= 1 {
        return 0;
    }
    let mut st = LazySegTree::<Affine>::new(m - 1);
    for i in 0..m - 1 {
        st.dat[st.n - 1 + i] = -(coord[i + 1] - coord[i]);
    }
    for i in (0..st.n - 1).rev() {
        st.dat[i] = st.dat[2 * i + 1] + st.dat[2 * i + 2];
    }
    let mut ev = vec![];
    for &(c, d, e, f) in a {
        if d >= f {
            continue;
        }
        let d = coord.binary_search(&d).unwrap();
        let f = coord.binary_search(&f).unwrap();
        ev.push((c, d, f));
        ev.push((e, d, f));
    }
    ev.sort_unstable();
    let mut last = 0;
    let mut tot = 0;
    let whole = coord[m - 1] - coord[0];
    for (c, l, r) in ev {
        tot += (st.query(0, m - 1) + whole) / 2 * (c - last);
        st.update(l, r, (-1, 0));
        last = c;
    }
    tot
}

fn main() {
    input! {
        n: usize,
        hwrc: [(i64, i64, i64, i64); n],
    }
    let mut ans = 0;
    for x in 0..2 {
        for y in 0..2 {
            let mut a = vec![];
            for &(h, w, r, c) in &hwrc {
                if (r + c + x + y) % 2 != 0 {
                    continue;
                }
                a.push(((r + 1 - x) / 2, (c + 1 - y) / 2,
                        (h + r + 1 - x) / 2, (w + c + 1 - y) / 2));
            }
            ans += calc(&a);
        }
    }
    println!("{}", ans);
}
