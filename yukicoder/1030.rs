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
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
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

struct LCA {
    st: Vec<usize>,
    par: Vec<usize>,
    jmp: Vec<usize>,
    dep: Vec<usize>,
}

// Constant-factor speedup used in https://codeforces.com/contest/1083/submission/46874242.
// Based on HL-decomposition.
// par[root] = root should hold.
// Verified by https://codeforces.com/contest/1083/submission/51934575.
impl LCA {
    // For each node, make the most heavy child the first child.
    fn dfs_left(ch: &mut [Vec<usize>], v: usize, sz: &mut [usize],
                dep: &mut [usize], d: usize) {
        dep[v] = d;
        let mut s = 1;
        for i in 0..ch[v].len() {
            let w = ch[v][i];
            Self::dfs_left(ch, w, sz, dep, d + 1);
            s += sz[w];
            if sz[w] > sz[ch[v][0]] {
                ch[v].swap(i, 0);
            }
        }
        sz[v] = s;
    }
    fn dfs(ch: &[Vec<usize>], st: &mut [usize], v: usize,
           cnt: &mut usize, jmp: &mut [usize]) {
        st[v] = *cnt;
        *cnt += 1;
        if ch[v].len() >= 1 {
            jmp[ch[v][0]] = jmp[v];
        }
        for &w in &ch[v] {
            Self::dfs(ch, st, w, cnt, jmp);
        }
    }
    fn new(ch: &mut [Vec<usize>], par: &[usize], root: usize) -> Self {
        let n = ch.len();
        let mut st = vec![0; n];
        let mut cnt = 0;
        let mut sz = vec![0; n];
        let mut jmp = vec![0; n];
        let mut dep = vec![0; n];
        Self::dfs_left(ch, root, &mut sz, &mut dep, 0);
        for i in 0..n {
            jmp[i] = i;
        }
        Self::dfs(ch, &mut st, root, &mut cnt, &mut jmp);
        LCA {
            st: st,
            par: par.to_vec(),
            jmp: jmp,
            dep: dep,
        }
    }
    fn lca(&self, mut x: usize, mut y: usize) -> usize {
        let jmp = &self.jmp;
        let st = &self.st;
        while jmp[x] != jmp[y] {
            if st[x] < st[y] {
                std::mem::swap(&mut x, &mut y);
            }
            x = self.par[jmp[x]];
        }
        if st[x] < st[y] {
            x
        } else {
            y
        }
    }
}

fn dfs(ch: &[Vec<usize>], ans: &mut [i64], c: &[i64], v: usize, x: i64) {
    let x = max(x, c[v]);
    ans[v] = x;
    for &w in &ch[v] {
        dfs(ch, ans, c, w, x);
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize, k: usize, q: usize,
        c: [i64; n],
        a: [usize1; k],
        ef: [(usize1, usize1); n - 1],
        t: [(i32, usize1, usize1); q],
    }
    let mut ch = vec![vec![]; n];
    let mut par = vec![0; n];
    for &(e, f) in &ef {
        ch[f].push(e);
        par[e] = f;
    }
    let lca = LCA::new(&mut ch, &par, 0);
    // precomp
    let mut ans = vec![0; n];
    dfs(&ch, &mut ans, &c, 0, 0);
    let mut st = SegTree::new(k, |x, y| {
        match (x, y) {
            (None, _) => y,
            (_, None) => x,
            (Some(x), Some(y)) => Some(lca.lca(x, y))
        }
    }, None);
    for i in 0..k {
        st.update(i, Some(a[i]));
    }
    for (kind, x, y) in t {
        if kind == 1 {
            st.update(x, Some(y));
        } else {
            let l = x;
            let r = y + 1;
            let v = st.query(l, r).unwrap();
            puts!("{}\n", ans[v]);
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
