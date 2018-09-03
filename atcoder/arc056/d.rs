#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::*;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}
fn get_word() -> String {
    let mut stdin = std::io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.is_err() || res.ok().unwrap() == 0 || u8b[0] <= ' ' as u8 {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = std::string::String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}
fn parse<T: std::str::FromStr>(s: &str) -> T { s.parse::<T>().ok().unwrap() }
/**
 * Lazy Segment Tree. This data structure is useful for fast folding and updating on intervals of an array
 * whose elements are elements of monoid T. Note that constructing this tree requires the identity
 * element of T and the operation of T. This is monomorphised, because of efficiency. T := i64, biop = max, upop = (+)
 * Reference: http://d.hatena.ne.jp/kyuridenamida/20121114/1352835261
 * Verified by ARC073-D (http://arc073.contest.atcoder.jp/submissions/1439847)
 */
pub struct LazySegTree<BiOp> {
    n: usize,
    dat: Vec<i64>,
    lazy: Vec<i64>,
    e: i64,
    biop: BiOp,
    upe: i64, // identity for upop
}

impl<BiOp: Fn(i64, i64) -> i64> LazySegTree<BiOp> {
    pub fn new(n_: usize, biop: BiOp, e: i64, upe: i64) -> Self {
    let mut n = 1;
    while n < n_ { n *= 2; } // n is a power of 2
        LazySegTree {n: n, dat: vec![e; 2 * n - 1], lazy: vec![upe; 2 * n - 1], e: e, biop: biop, upe: upe}
    }
    #[inline]
    fn lazy_evaluate_node(&mut self, k: usize) {
        self.dat[k] += self.lazy[k]; // TODO How do biop and upop interact? biop = max, upop = (+) are assumed
        if k < self.n - 1 {
            self.lazy[2 * k + 1] += self.lazy[k];
            self.lazy[2 * k + 2] += self.lazy[k];
        }
        self.lazy[k] = self.upe; // identity for upop
    }
    #[inline]
    fn update_node(&mut self, k: usize) {
        self.dat[k] = (self.biop)(self.dat[2 * k + 1], self.dat[2 * k + 2]);
    }
    fn update_sub(&mut self, a: usize, b: usize, v: i64, k: usize, l: usize, r: usize) {
        self.lazy_evaluate_node(k);

        // [a,b) and  [l,r) intersects?
        if r <= a || b <= l {return;}
        if a <= l && r <= b {
            self.lazy[k] += v;
            self.lazy_evaluate_node(k);
            return;
        }

        self.update_sub(a, b, v, 2 * k + 1, l, (l + r) / 2);
        self.update_sub(a, b, v, 2 * k + 2, (l + r) / 2, r);
        self.update_node(k);
    }
    /* ary[i] = upop(ary[i], v) for i in [a, b] (inclusive) */
    #[inline]
    pub fn update(&mut self, a: usize, b: usize, v: i64) {
        let n = self.n;
        self.update_sub(a, b + 1, v, 0, 0, n);
    }
    /* l,r are for simplicity */
    fn query_sub(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> i64 {
        self.lazy_evaluate_node(k);

        // [a,b) and  [l,r) intersect?
        if r <= a || b <= l {return self.e;}
        if a <= l && r <= b {return self.dat[k];}
        let vl = self.query_sub(a, b, 2 * k + 1, l, (l + r) / 2);
        let vr = self.query_sub(a, b, 2 * k + 2, (l + r) / 2, r);
        self.update_node(k);
        return (self.biop)(vl, vr);
    }
    /* [a, b] (note: inclusive) */
    #[inline]
    pub fn query(&mut self, a: usize, b: usize) -> i64 {
        let n = self.n;
        self.query_sub(a, b + 1, 0, 0, n)
    }
}


#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { parse(&get_word()) }

const W: usize = 1_000_100;
const INF: i64 = 1 << 50;
fn main() {
    let n: usize = get();
    let w: Vec<i64> = (0 .. n).map(|_| get()).collect();
    let mut st = LazySegTree::new(W, |x, y| max(x, y), -INF, 0);
    let mut dp: Vec<i64> = vec![-INF; W];
    st.update(0, 0, INF);
    dp[0] = 0;
    let mut ts: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut inv = vec![Vec::new(); W];
    for i in 0 .. n {
        let m: usize = get();
        for _ in 0 .. m {
            let t = get();
            ts[i].push(t);
            inv[t].push(i);
        }
    }
    let mut pos = vec![0_usize; n];
    for t in 2 .. W {
        for &idx in inv[t].iter() {
            let prev = if pos[idx] == 0 { 0 } else { ts[idx][pos[idx] - 1] };
            st.update(prev, t, w[idx]);
            pos[idx] += 1;
        }
        if t % 2 == 1 {
            let ma = st.query(0, t - 1);
            let cur = st.query(t, t);
            if cur != -INF {
                continue;
            }
            st.update(t, t, ma + INF);
            dp[t] = ma;
            /*
            if t <= 10 {
                eprintln!("dp = {:?}", &dp[0 .. t + 1]);
                for i in 0 .. t + 1 {
                    eprint!(" {}", st.query(i, i));
                }
                eprintln!();
            }
             */
        }
    }
    println!("{}", dp.iter().max().unwrap());
}
