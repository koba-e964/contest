#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Read, Write, BufWriter};
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
    pub fn bisect_from_beg_sub<F: Fn(I) -> bool>(&self, ask: &F, k: usize, l: usize, r: usize, acc: I) -> usize {
        if ask((self.op)(acc, self.dat[k])) {
            return r;
        }
        if r - l == 1 {
            return l;
        }
        let mid = (l + r) / 2;
        let sub0 = self.bisect_from_beg_sub(ask, 2 * k + 1, l, mid, acc);
        if sub0 != mid {
            return sub0;
        }
        let sub1 = self.bisect_from_beg_sub(ask, 2 * k + 2, mid, r, (self.op)(acc, self.dat[2 * k + 1]));
        sub1
    }
    pub fn bisect_from_beg<F: Fn(I) -> bool>(&self, ask: &F) -> usize {
        self.bisect_from_beg_sub(ask, 0, 0, self.n, self.e)
    }
}

struct LCA {
    st: Vec<usize>,
    par: Vec<usize>,
    jmp: Vec<usize>,
    dep: Vec<usize>,
}

// Constant-factor speedup used in https://codeforces.com/contest/1083/submission/46874242.
// Base on HL-decomposition.
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

#[derive(Clone, Copy)]
struct Dat<'a> {
    lca_q: &'a LCA,
    x: usize,
    y: usize,
}

// Reference: https://codeforces.com/contest/1083/submission/46867095
fn mul<'a>(a: Dat<'a>, b: Dat<'a>) -> Dat<'a> {
    let lca_q = a.lca_q;
    // mul with identity
    if a.x == usize::max_value() {
        return b;
    }
    if b.x == usize::max_value() {
        return a;
    }
    // mul with invalid value
    if a.y == usize::max_value() {
        return a;
    }
    if b.y == usize::max_value() {
        return b;
    }
    let void = Dat {
        lca_q: lca_q,
        x: 0,
        y: usize::max_value(),
    };
    let ends = [a.x, a.y, b.x, b.y];
    let mut l = [[0; 4]; 4];
    for i in 0..4 { l[i][i] = ends[i]; }
    for i in 0..4 {
        for j in i + 1..4 {
            l[i][j] = lca_q.lca(ends[i], ends[j]);
            l[j][i] = l[i][j];
        }
    }
    let mut d = [[0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            d[i][j] = lca_q.dep[ends[i]] + lca_q.dep[ends[j]] - 2 * lca_q.dep[l[i][j]];
        }
    }
    for i in 0..4 {
        for j in i + 1..4 {
            let mut ok = true;
            for k in 0..4 {
                if d[i][k] + d[k][j] != d[i][j] {
                    ok = false;
                    break;
                }
            }
            if ok {
                return Dat {
                    lca_q: lca_q,
                    x: ends[i],
                    y: ends[j],
                }
            }
        }
    }
    void
}

// This solution is implemented after the author read the editorial.
// O(N log N + Q log^2 N)
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    let n: usize = get();
    let mut p: Vec<usize> = (0..n).map(|_| get::<usize>()).collect();
    let d: Vec<usize> = (0..n - 1).map(|_| get::<usize>() - 1).collect();
    let mut invp = vec![0; n];
    for i in 0..n {
        invp[p[i]] = i;
    }
    let mut ch = vec![vec![]; n];
    let mut par = vec![0; n];
    for i in 1..n {
        ch[d[i - 1]].push(i);
        par[i] = d[i - 1];
    }
    let lca_q = LCA::new(&mut ch, &par, 0);
    let mut st = SegTree::new(n, mul, Dat {
        lca_q: &lca_q,
        x: usize::max_value(),
        y: n,
    });
    // init
    // optimized
    for i in 0..n {
        st.dat[i + st.n - 1] = Dat {
            lca_q: &lca_q,
            x: invp[i],
            y: invp[i],
        };
    }
    for i in (0..st.n - 1).rev() {
        st.dat[i] = mul(st.dat[2 * i + 1], st.dat[2 * i + 2]);
    }
    let q: usize = get();
    for _ in 0..q {
        let ty: i32 = get();
        if ty == 1 {
            let a: usize = get::<usize>() - 1;
            let b: usize = get::<usize>() - 1;
            if a != b {
                let pa = p[a];
                let pb = p[b];
                let tmpa = st.query(pa, pa + 1);
                let tmpb = st.query(pb, pb + 1);
                st.update(pa, tmpb);
                st.update(pb, tmpa);
                p.swap(a, b);
            }
        } else {
            let pass = st.bisect_from_beg(&|a: Dat| a.y != usize::max_value());
            puts!("{}\n", pass);
        }
    }
}

fn main() {
    solve();
}
