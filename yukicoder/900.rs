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

const B: usize = 3;

enum V {}

impl Action for V {
    type T = [i64; B]; // data
    type U = [[i64; B]; B]; // action
    fn update(x: Self::T, a: Self::U) -> Self::T {
        let mut ret = [0; B];
        for i in 0..B {
            for j in 0..B {
                ret[j] += x[i] * a[i][j];
            }
        }
        ret
    }
    fn upop(fst: Self::U, snd: Self::U) -> Self::U {
        let mut ret = [[0; B]; B];
        for i in 0..B {
            for j in 0..B {
                for k in 0..B {
                    ret[i][k] += fst[i][j] * snd[j][k];
                }
            }
        }
        ret
    }
    fn upe() -> Self::U { // identity for upop
        let mut a = [[0; B]; B];
        for i in 0..B {
            a[i][i] = 1;
        }
        a
    }
}

fn dfs(v: usize, ch: &[Vec<(usize, i64)>], di: i64, de: i64,
       dist: &mut [i64], dep: &mut [i64],
       cnt: &mut usize, rng: &mut [(usize, usize)]) {
    dist[v] = di;
    dep[v] = de;
    rng[v].0 = *cnt;
    *cnt += 1;
    for &(w, c) in &ch[v] {
        dfs(w, ch, di + c, de + 1, dist, dep, cnt, rng);
    }
    rng[v].1 = *cnt;
}

fn solve() {
    let n: usize = get();
    let mut ch = vec![vec![]; n];
    for _ in 0..n - 1 {
        let a = get::<usize>();
        let b = get::<usize>();
        let c: i64 = get();
        ch[a].push((b, c));
    }
    let mut dist = vec![0; n];
    let mut dep = vec![0; n];
    let mut rng = vec![(0, 0); n];
    let mut cnt = 0;
    dfs(0, &ch, 0, 0, &mut dist, &mut dep, &mut cnt, &mut rng);
    let mut arr = vec![[0; B]; n];
    for i in 0..n {
        let k = rng[i].0;
        arr[k] = [dist[i], dep[i], 1];
    }
    let mut st = DualSegTree::<V>::new(&arr);
    let q: usize = get();
    for _ in 0..q {
        let ty: i32 = get();
        let a: usize = get();
        if ty == 1 {
            let x: i64 = get();
            let (s, e) = rng[a];
            let d = dep[a];
            st.update(s, e, [
                [1, 0, 0],
                [x, 1, 0],
                [-x * d, 0, 1],
            ]);
        } else {
            println!("{}", st.query(rng[a].0)[0]);
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
