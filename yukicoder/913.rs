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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Verified by: https://atcoder.jp/contests/joisc2021/submissions/25693167
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
}

impl<R: Action> DualSegTree<R> {
    pub fn new(a: &[R::T]) -> Self {
        let n_ = a.len();
        let mut n = 1;
        while n < n_ { n *= 2; } // n is a power of 2
        DualSegTree {
            n: n,
            dat: a.to_vec(),
            lazy: vec![R::upe(); 2 * n - 1]
        }
    }
    #[inline]
    fn lazy_evaluate_node(&mut self, k: usize) {
        if self.lazy[k] == R::upe() { return; }
        if k >= self.n - 1 {
            let idx = k + 1 - self.n;
            self.dat[idx] = R::update(self.dat[idx], self.lazy[k]);
        }
        if k < self.n - 1 {
            self.lazy[2 * k + 1] = R::upop(self.lazy[2 * k + 1], self.lazy[k]);
            self.lazy[2 * k + 2] = R::upop(self.lazy[2 * k + 2], self.lazy[k]);
        }
        self.lazy[k] = R::upe(); // identity for upop
    }
    fn update_sub(&mut self, a: usize, b: usize, v: R::U, k: usize, l: usize, r: usize) {
        self.lazy_evaluate_node(k);

        // [a,b) and  [l,r) intersects?
        if r <= a || b <= l {return;}
        if a <= l && r <= b {
            self.lazy[k] = R::upop(self.lazy[k], v);
            self.lazy_evaluate_node(k);
            return;
        }

        self.update_sub(a, b, v, 2 * k + 1, l, (l + r) / 2);
        self.update_sub(a, b, v, 2 * k + 2, (l + r) / 2, r);
    }
    /* ary[i] = upop(ary[i], v) for i in [a, b) (half-inclusive) */
    #[inline]
    pub fn update(&mut self, a: usize, b: usize, v: R::U) {
        let n = self.n;
        self.update_sub(a, b, v, 0, 0, n);
    }
    /* l,r are for simplicity */
    fn update_at_sub(&mut self, a: usize, k: usize, l: usize, r: usize) {
        self.lazy_evaluate_node(k);

        // [a,a+1) and  [l,r) intersect?
        if r <= a || a + 1 <= l { return; }
        if a <= l && r <= a + 1 { return; }
        self.update_at_sub(a, 2 * k + 1, l, (l + r) / 2);
        self.update_at_sub(a, 2 * k + 2, (l + r) / 2, r);
    }
    /* [a, b) (note: half-inclusive) */
    #[inline]
    pub fn query(&mut self, a: usize) -> R::T {
        let n = self.n;
        self.update_at_sub(a, 0, 0, n);
        self.dat[a]
    }
}

enum Chmin {}

impl Action for Chmin {
    type T = i64; // data
    type U = i64; // action, a |-> x |-> min(x, a)
    fn update(x: Self::T, a: Self::U) -> Self::T {
        std::cmp::min(x, a)
    }
    fn upop(fst: Self::U, snd: Self::U) -> Self::U {
        std::cmp::min(fst, snd)
    }
    fn upe() -> Self::U { // identity for upop
        1 << 50
    }
}

/*
 * Online monotone minima dp. For example, monge dp can be efficiently computed
 * by online_dc.
 * Verified by: https://yukicoder.me/problems/no/705
 * submission: https://yukicoder.me/submissions/566775
 */
const INF: i64 = 1 << 60;

// Complexity: O(n log m + m) where n = r - l, m = b - a.
fn monotone_minima<F>(l: usize, r: usize, a: usize, b: usize,
                      lat: &mut [i64], realizer: &mut [usize],
                      cost_fun: &F)
where F: Fn(usize, usize) -> i64 {
    let n = r - l;
    let m = b - a;
    if n == 0 || m == 0 {
        return;
    }
    let mid = (a + b) / 2;
    let mut mi = (INF, n);
    for i in l..r {
        let cost = cost_fun(i, mid);
        mi = std::cmp::min(mi, (cost, i));
    }
    let idx = mi.1;
    assert!(l <= idx && idx < r);
    lat[mid] = std::cmp::min(lat[mid], mi.0);
    realizer[mid] = idx;
    monotone_minima(l, idx + 1, a, mid, lat, realizer, cost_fun);
    monotone_minima(idx, r, mid + 1, b, lat, realizer, cost_fun);
}

fn rec(l: usize, r: usize, a: &[i64], acc: &[i64],
       cons: &mut Vec<(usize, usize, i64)>) {
    if l >= r {
        return;
    }
    if l + 1 == r {
        cons.push((l, r, a[l] + 1));
        return;
    }
    let mid = (l + r) / 2;
    rec(l, mid, a, acc, cons);
    rec(mid, r, a, acc, cons);
    let mut dp = vec![INF; r - mid];
    let mut realizer = vec![0; r - mid];
    monotone_minima(0, mid - l, 0, r - mid, &mut dp, &mut realizer, &|i, j| {
        let ii = i as i64;
        let jj = (j + mid - l + 1) as i64;
        ii * ii - acc[l + i] + jj * jj + acc[j + mid + 1] - 2 * ii * jj
    });
    for i in 0..r - mid {
        cons.push((l + realizer[i], mid + 1 + i, dp[i]));
    }
    let mut dp = vec![INF; mid - l];
    let mut realizer = vec![0; mid - l];
    monotone_minima(0, r - mid, 0, mid - l, &mut dp, &mut realizer, &|j, i| {
        let ii = i as i64;
        let jj = (j + mid - l + 1) as i64;
        ii * ii - acc[l + i] + jj * jj + acc[j + mid + 1] - 2 * ii * jj
    });
    for i in 0..mid - l {
        cons.push((l + i, mid + 1 + realizer[i], dp[i]));
    }
}

fn calc(a: &[i64]) -> Vec<i64> {
    let n = a.len();
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + a[i];
    }
    let mut cons = vec![];
    rec(0, n, &a, &acc, &mut cons);
    let mut st = DualSegTree::<Chmin>::new(&vec![INF; n]);
    for (l, r, val) in cons {
        st.update(l, r, val);
    }
    let mut ret = vec![0; n];
    for i in 0..n {
        ret[i] = st.query(i);
    }
    ret
}

// https://yukicoder.me/problems/no/913 (4)
// monotone minima はうまくいかなかった。global min は正しく求まるが、それぞれの要素を含む min が正しくなかった。
// 分割統治の各ステップで、min(a[l..r]) = k という情報が得られるたびに範囲更新していけばよいのかな? 左右どちらからもやれば全範囲カバーできそう。
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        a: [i64; n],
    }
    let dp = calc(&a);
    for i in 0..n {
        puts!("{}\n", dp[i]);
    }
}
