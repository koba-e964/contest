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
    ($next:expr, (switch, $t1:tt, $t2:tt)) => {{
        let ty = read_value!($next, i32);
        if ty == 0 {
            Ok(read_value!($next, $t1))
        } else {
            Err(read_value!($next, $t2))
        }
    }};
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Bisect<T> {
    fn lower_bound(&self, val: &T) -> usize;
    fn upper_bound(&self, val: &T) -> usize;
}

impl<T: Ord> Bisect<T> for [T] {
    fn lower_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] >= val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
    fn upper_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] > val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
}

// Li-Chao segment tree for min.
// Ref: https://judge.yosupo.jp/submission/17300 by niuez
pub type LCTCoord = i64;
pub struct MinLiChaoSegTree {
    n: usize, // size of this Li-Chao segment tree
    xs: Box<[LCTCoord]>, // the coordinates associated to 0..n
    dat: Box<[(LCTCoord, LCTCoord)]>,
    e: (LCTCoord, LCTCoord), // identity for min
}

impl MinLiChaoSegTree {
    #[inline(always)]
    fn of((a, b): (LCTCoord, LCTCoord), x: LCTCoord) -> LCTCoord {
        a * x + b
    }
    // Initializes a Li-Chao segment tree with given coordinates and an identity for min.
    pub fn new(xs: &[LCTCoord], e: (LCTCoord, LCTCoord)) -> Self {
        assert!(!xs.is_empty());
        let n = xs.len().next_power_of_two();
        let mut xs = xs.to_vec();
        xs.resize(n, *xs.last().unwrap());
        MinLiChaoSegTree {
            n: n,
            xs: xs.into_boxed_slice(),
            dat: vec![e; 2 * n].into_boxed_slice(),
            e: e,
        }
    }
    // O(log n)
    fn add_range(&mut self, mut idx: usize, mut l: usize, mut r: usize, mut added: (LCTCoord, LCTCoord)) {
        let n = self.n;
        while idx < 2 * n {
            let m = (l + r) >> 1;
            let cur = self.dat[idx];
            let bl = Self::of(cur, self.xs[l]) > Self::of(added, self.xs[l]);
            let bm = Self::of(cur, self.xs[m]) > Self::of(added, self.xs[m]);
            let br = Self::of(cur, self.xs[r - 1]) > Self::of(added, self.xs[r - 1]);
            if bm {
                std::mem::swap(&mut self.dat[idx], &mut added);
            }
            if bl == br {
                // self.dat[idx] <= added for xs[l] <= x < xs[r]. No extra work needed.
                break;
            }
            if bl != bm {
                //        l m r
                // added: - + +
                // cur  : + - -
                // Recurses into [l, m).
                idx <<= 1;
                r = m; 
            } else {
                //        l m r
                // added: + + -
                // cur  : - - +
                // Recurses into [m, r).
                idx = 2 * idx + 1;
                l = m;
            }
        }
    }
    // lct.add_line((a, b)) adds a line y = ax + b.
    // O(log n)
    #[inline(always)]
    #[allow(unused)]
    pub fn add_line(&mut self, line: (LCTCoord, LCTCoord)) {
        self.add_range(1, 0, self.n, line);
    }
    // lct.add_seg(l..r, (a, b)) adds a segment y = ax + b, xs[l] <= x < xs[r].
    // O(log^2 n)
    pub fn add_seg(&mut self, rng: std::ops::Range<usize>, line: (LCTCoord, LCTCoord)) {
        let (mut l, mut r) = (rng.start, rng.end);
        debug_assert!(l <= r && r <= self.n);
        if l == r { return; }
        let mut li = l + self.n;
        let mut ri = r + self.n;
        let mut len = 1;
        while li < ri {
            if (li & 1) != 0 {
                self.add_range(li, l, l + len, line);
                li += 1;
                l += len;
            }
            if (ri & 1) != 0 {
                ri -= 1;
                self.add_range(ri, r - len, r, line);
                r -= len;
            }
            li >>= 1;
            ri >>= 1;
            len <<= 1;
        }
    }
    // lct.get(x) gets the minimum value of the lines added so far at xs[x].
    // O(log n)
    pub fn get(&self, x: usize) -> LCTCoord {
        let mut idx = x + self.n;
        let x = self.xs[x];
        let mut res = Self::of(self.e, x);
        while idx > 0 {
            let cur = Self::of(self.dat[idx], x);
            if cur < res {
                res = cur;
            }
            idx >>= 1;
        }
        res
    }
}

const REPEAT: bool = false;

// REPEAT=false: 435ms (https://judge.yosupo.jp/submission/125824)
// REPEAT=true: 1882ms (https://judge.yosupo.jp/submission/125821)
// computation: 160ms, I/O: 275ms
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, q: usize,
        lrab: [(i64, i64, i64, i64); n],
        qs: [(switch, (i64, i64, i64, i64), i64); q],
    }
    let mut xs = Vec::with_capacity(q);
    // Look-ahead of queries
    for &q in &qs {
        if let Err(x) = q {
            xs.push(x);
        }
    }
    if xs.is_empty() {
        return;
    }
    xs.sort(); xs.dedup();
    const INF: i64 = 1 << 61;
    let mut lct = MinLiChaoSegTree::new(&xs, (0, INF));
    for &(l, r, a, b) in &lrab {
        let l = xs.lower_bound(&l);
        let r = xs.lower_bound(&r);
        lct.add_seg(l..r, (a, b));
    }
    for &q in &qs {
        match q {
            Ok((l, r, a, b)) => {
                let l = xs.lower_bound(&l);
                let r = xs.lower_bound(&r);
                lct.add_seg(l..r, (a, b));
            }
            Err(x) => {
                let x = xs.binary_search(&x).unwrap();
                let ans = lct.get(x);
                if ans >= INF {
                    puts!("INFINITY\n");
                } else {
                    puts!("{}\n", ans);
                }
            }
        }
    }
    if REPEAT {
        for _ in 1..10 {
            let mut lct = MinLiChaoSegTree::new(&xs, (0, INF));
            for &(l, r, a, b) in &lrab {
                let l = xs.lower_bound(&l);
                let r = xs.lower_bound(&r);
                lct.add_seg(l..r, (a, b));
            }
            for &q in &qs {
                match q {
                    Ok((l, r, a, b)) => {
                        let l = xs.lower_bound(&l);
                        let r = xs.lower_bound(&r);
                        lct.add_seg(l..r, (a, b));
                    }
                    Err(x) => {
                        let x = xs.binary_search(&x).unwrap();
                        lct.get(x);
                    }
                }
            }    
        }
    }
}
