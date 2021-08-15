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

/**
 * Sparse Table.
 * BiOp should be the type of a binary operator which is
 * associative, commutative and idempotent.
 * (For example, both min and gcd satisfy these properties.)
 * Verified by: AtCoder CODE FESTIVAL 2016 Tournament Round 3 (Parallel) B
 * (http://cf16-tournament-round3-open.contest.atcoder.jp/submissions/1026294)
 */
struct SparseTable<T> {
    st: Vec<Vec<T>>,
}

impl<T> SparseTable<T>
    where T: Copy + Ord {
    pub fn new(ary: &[T]) -> Self {
        let n = ary.len();
        let mut h = 1;
        while 1 << h < n {
            h += 1;
        }
        let mut st: Vec<Vec<T>> = vec![Vec::from(ary); h + 1];
        for i in 0 .. n {
            st[0][i] = ary[i];
        }
        for b in 1 .. (h + 1) {
            if n + 1 < 1 << b {
                break;
            }
            for i in 0 .. (n + 1 - (1 << b)) {
                let next_idx = (1 << (b - 1)) + i;
                st[b][i] = std::cmp::min(st[b - 1][i], st[b - 1][next_idx]);
            }
        }
        SparseTable {st: st}
    }
    fn top_bit(t: usize) -> usize {
        8 * std::mem::size_of::<usize>() - 1 - t.leading_zeros() as usize
    }
    pub fn query(&self, f: usize, s: usize) -> T {
        assert!(f <= s);
        let b = Self::top_bit(s + 1 - f);
        let endpoint = s + 1 - (1 << b);
        unsafe {
            let x = self.st.get_unchecked(b);
            std::cmp::min(*x.get_unchecked(f), *x.get_unchecked(endpoint))
        }
    }
}

struct SegTree<I> {
    n: usize,
    dat: Vec<I>,
    e: I,
}

impl<I> SegTree<I>
    where I: Copy + Ord {
    pub fn new(n_: usize, e: I) -> Self {
        let mut n = 1;
        while n < n_ { n *= 2; } // n is a power of 2
        SegTree {n: n, dat: vec![e; 2 * n - 1], e: e}
    }
    /* [a, b) (note: half-inclusive)
     * http://proc-cpuinfo.fixstars.com/2017/07/optimize-segment-tree/ */
    pub fn query(&self, mut a: usize, mut b: usize) -> I {
        let mut val = self.e;
        a += self.n - 1;
        b += self.n - 1;
        while a < b {
            if (a & 1) == 0 {
                val = std::cmp::min(val, self.dat[a]);
            }
            if (b & 1) == 0 {
                val = std::cmp::min(self.dat[b - 1], val);
            }
            a = a / 2;
            b = (b - 1) / 2;
        }
        val
    }
}

fn prepare_rmq<T: Copy + Ord>(
    a: &[T], e: T,
) -> SegTree<T> {
    let n = a.len();
    let mut st = SegTree::new(n, e);
    for i in 0..n {
        st.dat[st.n - 1 + i] = a[i];
    }
    for i in (0..st.n - 1).rev() {
        st.dat[i] = std::cmp::min(st.dat[2 * i + 1], st.dat[2 * i + 2]);
    }
    st
}

// Ref: https://cp-algorithms.com/graph/rmq_linear.html
// Returns (root, edges to children)
// TODO: take F
fn build_cartesian_tree<Lt: Fn(i64, i64) -> bool>(
    a: &[i64], f: Lt,
) -> (usize, Vec<Vec<usize>>) {
    let n = a.len();
    let mut par = vec![n; n];
    let mut ch = vec![vec![]; n];
    let mut st = vec![];
    for i in 0..n {
        let mut last = n;
        while !st.is_empty() && !f(a[*st.last().unwrap()], a[i]) {
            last = st.pop().unwrap();
        }
        if let Some(&x) = st.last() {
            par[i] = x;
        }
        if last < n {
            par[last] = i;
        }
        st.push(i);
    }
    for i in 0..n {
        if par[i] < n {
            ch[par[i]].push(i);
        }
    }
    (st[0], ch)
}

fn euler_tour(root: usize, ch: &[Vec<usize>])
              -> (Vec<usize>, Vec<usize>, Vec<i32>) {
    let mut ans = vec![];
    let mut rng = vec![0; ch.len()];
    let mut to = vec![];
    fn dfs(v: usize, ch: &[Vec<usize>],
           ans: &mut Vec<i32>, rng: &mut [usize], to: &mut Vec<usize>,
           now: i32) {
        ans.push(now);
        rng[v] = to.len();
        to.push(v);
        for &w in &ch[v] {
            dfs(w, ch, ans, rng, to, now + 1);
            ans.push(now);
            to.push(v);
        }
    }
    dfs(root, &ch, &mut ans, &mut rng, &mut to, 0);
    (rng, to, ans)
}

// plus-minus 1 range min query
struct P1RMinQ {
    c: usize,
    a: Vec<i32>,
    st: SegTree<(i32, usize)>,
    tbl: Vec<Vec<Vec<(i32, usize)>>>,
    kind: Vec<usize>,
}

impl P1RMinQ {
    fn new(a: &[i32]) -> Self {
        let mut w = 1;
        let mut c = 1;
        while w * w <= a.len() {
            w *= 2;
            c += 1;
        }
        Self::with_block_size(a, c - 1)
    }
    fn with_block_size(a: &[i32], c: usize) -> Self {
        let n = a.len();
        let w = 1 << (c - 1);
        let mut tbl = vec![vec![vec![(0, 0); w]; c + 1]; c];
        let mut b = vec![0; c];
        for bits in 0..w {
            for i in 0..c - 1 {
                b[i + 1] = b[i] + if (bits & 1 << i) != 0 { 1 } else { -1 };
            }
            for i in 0..c {
                let mut tmp = (b[i], i);
                for j in i + 1..c + 1 {
                    tmp = std::cmp::min(tmp, (b[j - 1], j - 1));
                    tbl[i][j][bits] = tmp;
                }
            }
        }
        let mut sub = vec![(0, 0); (n + c - 1) / c];
        let mut kind = vec![0; (n + c - 1) / c];
        for i in 0..(n + c - 1) / c {
            let l = i * c;
            let r = std::cmp::min(i * c + c, n);
            sub[i] = (a[l], l);
            for j in l + 1..r {
                sub[i] = std::cmp::min(sub[i], (a[j], j));
                assert!(a[j] - a[j - 1] == 1 || a[j] - a[j - 1] == -1);
                if a[j] - a[j - 1] == 1 {
                    kind[i] |= 1 << (j - l - 1);
                }
            }
        }
        let st = prepare_rmq(&sub, (1 << 30, 0));
        P1RMinQ {
            c: c,
            a: a.to_vec(),
            st: st,
            tbl: tbl,
            kind: kind,
        }
    }
    fn query(&self, a: usize, b: usize) -> (i32, usize) {
        assert!(a < b);
        let c = self.c;
        let l = (a + c - 1) / c;
        let r = b / c;
        if l > r {
            let start = self.a[r * c];
            let sub = self.tbl[a - r * c][b - r * c][self.kind[r]];
            return (start + sub.0, r * c + sub.1);
        }
        let mut tmp = (std::i32::MAX, 0);
        if a < l * c {
            let start = self.a[(l - 1) * c];
            let sub = self.tbl[a - (l - 1) * c][c][self.kind[l - 1]];
            tmp = std::cmp::min(tmp, (start + sub.0, (l - 1) * c + sub.1));
        }
        if r * c < b {
            let start = self.a[r * c];
            let sub = self.tbl[0][b - r * c][self.kind[r]];
            tmp = std::cmp::min(tmp, (start + sub.0, r * c + sub.1));
        }
        if l < r {
            tmp = std::cmp::min(tmp, self.st.query(l, r));
        }
        tmp
    }
}

// Range max/min query with <O(n), O(1)> complexity
struct RMQ {
    inner: P1RMinQ,
    a: Vec<i64>,
    rng: Vec<usize>,
    to: Vec<usize>,
}

impl RMQ {
    fn new<Lt: Fn(i64, i64) -> bool>(a: &[i64], lt: Lt) -> Self {
        let (r, ct) = build_cartesian_tree(a, lt);
        let (rng, to, del) = euler_tour(r, &ct);
        let inner = P1RMinQ::new(&del);
        RMQ {
            inner: inner,
            a: a.to_vec(),
            rng: rng,
            to: to,
        }
    }
    fn query(&self, a: usize, b: usize) -> (i64, usize) {
        let sta = self.rng[a];
        let enb = self.rng[b - 1];
        let (st, en) = if sta < enb {
            (sta, enb)
        } else {
            (enb, sta)
        };
        let (_, idx) = self.inner.query(st, en + 1);
        let idx = self.to[idx];
        (self.a[idx], idx)
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, q: usize,
        a: [i64; n],
        lr: [(usize, usize); q],
    }
    let rmq = RMQ::new(&a, |x, y| x < y);
    for (l, r) in lr {
        puts!("{}\n", rmq.query(l, r).0);
    }
}
