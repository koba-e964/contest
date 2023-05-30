use std::cmp::*;
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

// Z algorithm. Calculates an array a[i] = |lcp(s, &s[i..])|,
// where s is the given slice.
// If n = s.length(), the returned array has length n + 1.
// E.g. z_algorithm(b"ababa") = vec![5, 0, 3, 0, 1, 0]
// Reference: http://snuke.hatenablog.com/entry/2014/12/03/214243
// Verified by: ABC284-F (https://atcoder.jp/contests/abc284/submissions/38752029)
fn z_algorithm<T: PartialEq>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut ret = vec![0; n + 1];
    ret[0] = n;
    let mut i = 1; let mut j = 0;
    while i < n {
        while i + j < n && s[j] == s[i + j] { j += 1; }
        ret[i] = j;
        if j == 0 { i += 1; continue; }
        let mut k = 1;
        while i + k < n && k + ret[k] < j {
            ret[i + k] = ret[k];
            k += 1;
        }
        i += k; j -= k;
    }
    ret
}

// Li-Chao segment tree for min.
// Ref: https://judge.yosupo.jp/submission/17300 by niuez
// Verified by: https://judge.yosupo.jp/problem/segment_add_get_min (https://judge.yosupo.jp/submission/125824)
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

// https://yukicoder.me/problems/no/2332 (3)
// L(l) := lcp(B[l..], A) の長さ とする。(Z-algorithm で前計算をすれば O(1) 時間で計算できる。)
// dp[l+u].chmin(dp[l] + C_l * u) (1 <= u <= L(l)) という DPを考えると、これはイベントの管理を行えば実行できる。
// もらう DP をする。Li-Chao tree で以下のイベントの処理ができれば良い。
// 1. x = i における線分たちの最小値を取得する。
// 2. 線分 i <= x <= i + L(i), y = C_i * (x - i) + dp[i] を追加する。
// 計算量は O(N + M log^2 M)
// Tags: li-chao-segment-trees
fn main() {
    input! {
        n: usize, m: usize,
        a: [u32; n],
        b: [u32; m],
        c: [i64; m],
    }
    let mut cont = a.clone();
    cont.extend_from_slice(&b);
    let z = z_algorithm(&cont);
    let mut xs = vec![];
    for i in 0..m + 1 {
        xs.push(i as i64);
    }
    const INF: i64 = 1 << 50;
    let mut lct = MinLiChaoSegTree::new(&xs, (0, INF));
    let mut dp = vec![0; m + 1];
    for i in 0..m + 1 {
        if i > 0 {
            dp[i] = lct.get(i);
        }
        if i < m {
            let len = min(n, z[n + i]);
            lct.add_seg(i..i + len + 1, (c[i], dp[i] - c[i] * i as i64));
        }
    }
    println!("{}", if dp[m] >= INF { -1 } else { dp[m] });
}
