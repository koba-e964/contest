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

/**
 * Union-Find tree.
 * Verified by https://atcoder.jp/contests/keyence2019/submissions/4071067
 */
struct UnionFind { disj: Vec<usize>, rank: Vec<usize> }

impl UnionFind {
    fn new(n: usize) -> Self {
        let disj = (0..n).collect();
        UnionFind { disj: disj, rank: vec![0; n] }
    }
    fn root(&mut self, x: usize) -> usize {
        if x != self.disj[x] {
            let par = self.disj[x];
            let r = self.root(par);
            self.disj[x] = r;
        }
        self.disj[x]
    }
    fn unite(&mut self, x: usize, y: usize) {
        let mut x = self.root(x);
        let mut y = self.root(y);
        if x == y { return }
        if self.rank[x] > self.rank[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.disj[x] = y;
        self.rank[y] = std::cmp::max(self.rank[y], self.rank[x] + 1);
    }
    #[allow(unused)]
    fn is_same_set(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
}

// Verified by https://atcoder.jp/contests/arc084/submissions/3935443
#[derive(Clone)]
struct BitSet {
    size: usize,
    buf: Vec<usize>,
}

impl BitSet {
    // size should be a multiple of bit-size of usize.
    fn new(size: usize) -> Self {
        let w = 8 * std::mem::size_of::<usize>();
        assert_eq!(size & (w - 1), 0);
        let count = size / w;
        BitSet {
            size: size,
            buf: vec![0; count],
        }
    }
    #[allow(unused)]
    fn set(&mut self, idx: usize, val: bool) {
        debug_assert!(idx < self.size);
        let w = 8 * std::mem::size_of::<usize>();
        let idx0 = idx / w;
        let idx1 = idx & (w - 1);
        if val {
            self.buf[idx0] |= 1 << idx1;
        } else {
            self.buf[idx0] &= !(1 << idx1);
        }
    }
    #[allow(unused)]
    fn get(&self, idx: usize) -> bool {
        let w = 8 * std::mem::size_of::<usize>();
        debug_assert!(idx < self.size);
        let idx0 = idx / w;
        let idx1 = idx & (w - 1);
        (self.buf[idx0] >> idx1 & 1) == 1
    }
    #[allow(unused)]
    fn shl(&self, val: usize) -> Self {
        if val >= self.size { return Self::new(self.size); }
        let w = 8 * std::mem::size_of::<usize>();
        let count = self.size / w;
        let sh0 = val / w;
        let sh1 = val & (w - 1);
        let mut ans = Self::new(self.size);
        if sh1 == 0 {
            for i in 0 .. count - sh0 {
                ans.buf[i + sh0] = self.buf[i];
            }
        } else {
            ans.buf[sh0] = self.buf[0] << sh1;
            for i in 1 .. count - sh0 {
                ans.buf[i + sh0] = self.buf[i] << sh1
                    | self.buf[i - 1] >> (w - sh1);
            }
        }
        ans
    }
    // Not verified
    #[allow(unused)]
    fn shr(&self, val: usize) -> Self {
        if val >= self.size { return Self::new(self.size); }
        let w = 8 * std::mem::size_of::<usize>();
        let count = self.size / w;
        let sh0 = val / w;
        let sh1 = val & (w - 1);
        let mut ans = Self::new(self.size);
        if sh1 == 0 {
            for i in 0 .. count - sh0 {
                ans.buf[i] = self.buf[i + sh0];
            }
        } else {
            for i in 0 .. count - sh0 - 1 {
                ans.buf[i] = self.buf[i + sh0] >> sh1
                    | self.buf[i + sh0 + 1] << (w - sh1);
            }
            ans.buf[self.size - sh0 - 1] = self.buf[self.size - 1] >> sh1;
        }
        ans
    }
    #[allow(unused)]
    fn msb(&self) -> Option<usize> {
        let w = 8 * std::mem::size_of::<usize>();
        let count = self.size / w;
        for i in (0 .. count).rev() {
            let v = self.buf[i];
            if v != 0 {
                return Some(w * i + w - 1 - v.leading_zeros() as usize);
            }
        }
        None
    }
}

// TODO reference is not allowed as rhs
impl std::ops::BitXorAssign for BitSet {
    fn bitxor_assign(&mut self, other: BitSet) {
        debug_assert_eq!(self.size, other.size);
        for i in 0 .. self.buf.len() {
            self.buf[i] ^= other.buf[i];
        }
    }
}
// TODO reference is not allowed as rhs
impl std::ops::BitOrAssign for BitSet {
    fn bitor_assign(&mut self, other: BitSet) {
        debug_assert_eq!(self.size, other.size);
        for i in 0 .. self.buf.len() {
            self.buf[i] |= other.buf[i];
        }
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize, r: usize,
        a: [usize1; n],
    }
    let mut uf = UnionFind::new(n);
    for i in 0..n {
        uf.unite(i, a[i]);
    }
    let mut occ = vec![0; n];
    for i in 0..n {
        occ[uf.root(i)] += 1;
    }
    const W: usize = 300_000 / 64 * 64 + 64;
    let mut bs = BitSet::new(W);
    bs.set(0, true);
    let mut num = vec![];
    let mut one = 0;
    for i in 0..n {
        if occ[i] == 0 { continue; }
        if occ[i] == 1 {
            one += 1;
        } else {
            num.push(occ[i]);
        }
    }
    for n in num {
        let tmp = bs.shl(n);
        bs |= tmp;
    }
    let mut ok = false;
    for i in 0..one + 1 {
        if r >= i && bs.get(r - i) {
            ok = true;
        }
    }
    puts!("{}\n", if ok { "Yes" } else { "No" });
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
