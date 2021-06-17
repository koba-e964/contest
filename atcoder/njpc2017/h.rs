#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

/**
 * Segment Tree. This data structure is useful for fast folding on intervals of an array
 * whose elements are elements of monoid I. Note that constructing this tree requires the identity
 * element of I and the operation of I.
 * Verified by: yukicoder No. 259 (http://yukicoder.me/submissions/100581)
 *              AGC015-E (http://agc015.contest.atcoder.jp/submissions/1461001)
 */
struct SegTree<I, BiOp> {
    n: usize,
    dat: Vec<I>,
    op: BiOp,
    e: I,
}

impl<I, BiOp> SegTree<I, BiOp>
    where BiOp: Fn(I, I) -> I,
          I: Copy {
    pub fn new(n_: usize, op: BiOp, e: I) -> Self {
        let mut n = 1;
        while n < n_ { n *= 2; } // n is a power of 2
        SegTree {n: n, dat: vec![e; 2 * n - 1], op: op, e: e}
    }
    /* ary[k] <- v */
    pub fn update(&mut self, idx: usize, v: I) {
        let mut k = idx + self.n - 1;
        self.dat[k] = v;
        while k > 0 {
            k = (k - 1) / 2;
            self.dat[k] = (self.op)(self.dat[2 * k + 1], self.dat[2 * k + 2]);
        }
    }
    /* [a, b) (note: half-inclusive)
     * http://proc-cpuinfo.fixstars.com/2017/07/optimize-segment-tree/ */
    pub fn query(&self, mut a: usize, mut b: usize) -> I {
        let mut left = self.e;
        let mut right = self.e;
        a += self.n - 1;
        b += self.n - 1;
        while a < b {
            if (a & 1) == 0 {
                left = (self.op)(left, self.dat[a]);
            }
            if (b & 1) == 0 {
                right = (self.op)(self.dat[b - 1], right);
            }
            a = a / 2;
            b = (b - 1) / 2;
        }
        (self.op)(left, right)
    }
}

// https://ei1333.github.io/luzhiled/snippets/tree/heavy-light-decomposition.html
// Verified by: NUPC2017 H
// https://atcoder.jp/contests/njpc2017/submissions/23535017
struct HLDecomp {
    euler: Vec<usize>,
    head: Vec<usize>,
    rev: Vec<usize>,
    par: Vec<usize>,
}

impl HLDecomp {
    fn dfs_sz(v: usize, p: usize, g: &mut [Vec<usize>], sz: &mut [usize],
              par: &mut [usize]) {
        par[v] = p;
        sz[v] = 1;
        if g[v].get(0) == Some(&p) {
            let last = g[v].len() - 1;
            g[v].swap(0, last);
        }
        for i in 0..g[v].len() {
            let to = g[v][i];
            if to == p {
                continue;
            }
            Self::dfs_sz(to, v, g, sz, par);
            sz[v] += sz[to];
            if sz[g[v][0]] < sz[to] {
                g[v].swap(0, i);
            }
        }
    }
    fn dfs_euler(v: usize, par: usize, g: &[Vec<usize>],
                 euler: &mut [usize], count: &mut usize,
                 head: &mut [usize], rev: &mut [usize]) {
        euler[v] = *count;
        *count += 1;
        rev[euler[v]] = v;
        for &to in &g[v] {
            if to == par {
                continue;
            }
            head[to] = if g[v][0] == to { head[v] } else { to };
            Self::dfs_euler(to, v, g, euler, count, head, rev);
        }
    }
    pub fn new(g: &[Vec<usize>]) -> Self {
        let mut g = g.to_vec();
        let n = g.len();
        let mut sz = vec![0; n];
        let mut par = vec![0; n];
        Self::dfs_sz(0, n, &mut g, &mut sz, &mut par);
        let mut euler = vec![0; n];
        let mut count = 0;
        let mut head = vec![0; n];
        let mut rev = vec![0; n];
        Self::dfs_euler(0, n, &g, &mut euler, &mut count, &mut head, &mut rev);
        HLDecomp {
            euler: euler,
            head: head,
            rev: rev,
            par: par,
        }
    }
    #[allow(unused)]
    pub fn get_id(&self, v: usize) -> usize {
        self.euler[v]
    }
    #[allow(unused)]
    pub fn from_id(&self, id: usize) -> usize {
        self.rev[id]
    }
    // M: commutative
    // M must not panic.
    #[allow(unused)]
    pub fn query<T, F: FnMut(usize, usize) -> T, M: Fn(T, T) -> T>(&self, mut u: usize, mut v: usize, mut f: F, mut m: M, e: T, edge: bool) -> T {
        let mut ans = e;
        self.divide(u, v, |l, r| {
            let ptr: *mut T = &mut ans;
            unsafe {
                let val = f(l, r);
                let ans = std::ptr::read(ptr);
                std::ptr::write(ptr, m(ans, val))
            }
        }, edge);
        ans
    }
    pub fn divide<F: FnMut(usize, usize)>(&self, mut u: usize, mut v: usize, mut f: F, edge: bool) {
        let euler = &self.euler;
        let head = &self.head;
        loop {
            if euler[u] > euler[v] {
                std::mem::swap(&mut u, &mut v);
            }
            if head[u] == head[v] {
                break;
            }
            f(euler[head[v]], euler[v] + 1);
            v = self.par[head[v]];
        }
        f(euler[u] + if edge { 1 } else { 0 }, euler[v] + 1);
    }
}

fn dfs(v: usize, par: usize, g: &[Vec<usize>], c: &[i32], dat: &mut [i32]) {
    let n = g.len();
    if par < n {
        dat[v] = c[v] ^ c[par] ^ 1;
    }
    for &w in &g[v] {
        if par == w {
            continue;
        }
        dfs(w, v, g, c, dat);
    }
}

// Tags: hl-decomposition
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        p: [usize1; n - 1],
        c: [i32; n],
        qs: [[usize1]],
    }
    let mut dat = vec![0; n];
    let mut g = vec![vec![]; n];
    for i in 0..n - 1 {
        g[i + 1].push(p[i]);
        g[p[i]].push(i + 1);
    }
    dfs(0, n, &g, &c, &mut dat);
    eprintln!("dat = {:?}", dat);
    let hld = HLDecomp::new(&g);
    let mut st = SegTree::new(n, |x, y| x | y, false);
    for i in 0..n {
        let id = hld.get_id(i);
        st.update(id, dat[i] == 1);
    }
    for q in qs {
        let u = q[0];
        if q.len() == 1 {
            let id = hld.get_id(u);
            st.update(id, !st.query(id, id + 1));
        } else {
            let v = q[1];
            if hld.query(u, v, |l, r| st.query(l, r), |a, b| a | b, false, true) {
                puts!("NO\n");
            } else {
                puts!("YES\n");
            }
        }
    }
}
