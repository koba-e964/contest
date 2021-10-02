#[allow(unused_imports)]
use std::cmp::*;
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

// https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/string.rs
// Verified by: https://atcoder.jp/contests/abc213/submissions/25662432
fn sa_naive<T: Ord>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut sa: Vec<usize> = (0..n).collect();
    sa.sort_by(|&(mut l), &(mut r)| {
        if l == r {
            return std::cmp::Ordering::Equal;
        }
        while l < n && r < n {
            if s[l] != s[r] {
                return s[l].cmp(&s[r]);
            }
            l += 1;
            r += 1;
        }
        if l == n {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    sa
}

fn sa_doubling(s: &[i32]) -> Vec<usize> {
    let n = s.len();
    let mut sa: Vec<usize> = (0..n).collect();
    let mut rnk: Vec<i32> = s.to_vec();
    let mut tmp = vec![0; n];
    let mut k = 1;
    while k < n {
        let cmp = |&x: &usize, &y: &usize| {
            if rnk[x] != rnk[y] {
                return rnk[x].cmp(&rnk[y]);
            }
            let rx = if x + k < n { rnk[x + k] } else { -1 };
            let ry = if y + k < n { rnk[y + k] } else { -1 };
            rx.cmp(&ry)
        };
        sa.sort_by(cmp);
        tmp[sa[0]] = 0;
        for i in 1..n {
            tmp[sa[i]] = tmp[sa[i - 1]]
                + if cmp(&sa[i - 1], &sa[i]) == std::cmp::Ordering::Less {
                    1
                } else {
                    0
                };
        }
        std::mem::swap(&mut tmp, &mut rnk);
        k *= 2;
    }
    sa
}

trait Threshold {
    fn threshold_naive() -> usize;
    fn threshold_doubling() -> usize;
}

enum DefaultThreshold {}
impl Threshold for DefaultThreshold {
    fn threshold_naive() -> usize {
        10
    }
    fn threshold_doubling() -> usize {
        40
    }
}

// |returned| = |s|
// Complexity: O(|s| upper)
#[allow(clippy::cognitive_complexity)]
fn sa_is<T: Threshold>(s: &[usize], upper: usize) -> Vec<usize> {
    let n = s.len();
    match n {
        0 => return vec![],
        1 => return vec![0],
        2 => return if s[0] < s[1] { vec![0, 1] } else { vec![1, 0] },
        _ => (),
    }
    if n < T::threshold_naive() {
        return sa_naive(s);
    }
    if n < T::threshold_doubling() {
        let s: Vec<i32> = s.iter().map(|&x| x as i32).collect();
        return sa_doubling(&s);
    }
    let mut sa = vec![0; n];
    let mut ls = vec![false; n];
    for i in (0..n - 1).rev() {
        ls[i] = if s[i] == s[i + 1] {
            ls[i + 1]
        } else {
            s[i] < s[i + 1]
        };
    }
    let mut sum_l = vec![0; upper + 1];
    let mut sum_s = vec![0; upper + 1];
    for i in 0..n {
        if !ls[i] {
            sum_s[s[i]] += 1;
        } else {
            sum_l[s[i] + 1] += 1;
        }
    }
    for i in 0..=upper {
        sum_s[i] += sum_l[i];
        if i < upper {
            sum_l[i + 1] += sum_s[i];
        }
    }

    // sa's origin is 1.
    let induce = |sa: &mut [usize], lms: &[usize]| {
        for elem in sa.iter_mut() {
            *elem = 0;
        }
        let mut buf = sum_s.clone();
        for &d in lms {
            if d == n {
                continue;
            }
            let old = buf[s[d]];
            buf[s[d]] += 1;
            sa[old] = d + 1;
        }
        buf.copy_from_slice(&sum_l);
        let old = buf[s[n - 1]];
        buf[s[n - 1]] += 1;
        sa[old] = n;
        for i in 0..n {
            let v = sa[i];
            if v >= 2 && !ls[v - 2] {
                let old = buf[s[v - 2]];
                buf[s[v - 2]] += 1;
                sa[old] = v - 1;
            }
        }
        buf.copy_from_slice(&sum_l);
        for i in (0..n).rev() {
            let v = sa[i];
            if v >= 2 && ls[v - 2] {
                buf[s[v - 2] + 1] -= 1;
                sa[buf[s[v - 2] + 1]] = v - 1;
            }
        }
    };
    // origin: 1
    let mut lms_map = vec![0; n + 1];
    let mut m = 0;
    for i in 1..n {
        if !ls[i - 1] && ls[i] {
            lms_map[i] = m + 1;
            m += 1;
        }
    }
    let mut lms = Vec::with_capacity(m);
    for i in 1..n {
        if !ls[i - 1] && ls[i] {
            lms.push(i);
        }
    }
    assert_eq!(lms.len(), m);
    induce(&mut sa, &lms);

    if m > 0 {
        let mut sorted_lms = Vec::with_capacity(m);
        for &v in &sa {
            if lms_map[v - 1] != 0 {
                sorted_lms.push(v - 1);
            }
        }
        let mut rec_s = vec![0; m];
        let mut rec_upper = 0;
        rec_s[lms_map[sorted_lms[0]] - 1] = 0;
        for i in 1..m {
            let mut l = sorted_lms[i - 1];
            let mut r = sorted_lms[i];
            let end_l = if lms_map[l] < m { lms[lms_map[l]] } else { n };
            let end_r = if lms_map[r] < m { lms[lms_map[r]] } else { n };
            let same = if end_l - l != end_r - r {
                false
            } else {
                while l < end_l {
                    if s[l] != s[r] {
                        break;
                    }
                    l += 1;
                    r += 1;
                }
                l != n && s[l] == s[r]
            };
            if !same {
                rec_upper += 1;
            }
            rec_s[lms_map[sorted_lms[i]] - 1] = rec_upper;
        }

        let rec_sa = sa_is::<T>(&rec_s, rec_upper);
        for i in 0..m {
            sorted_lms[i] = lms[rec_sa[i]];
        }
        induce(&mut sa, &mut sorted_lms);
    }
    for elem in sa.iter_mut() {
        *elem -= 1;
    }
    sa
}

fn suffix_array_lowercase(s: &[char]) -> Vec<usize> {
    let s: Vec<usize> = s.iter().map(|&x| (x as u8 - b'a') as usize).collect();
    sa_is::<DefaultThreshold>(&s, 25)
}

struct LCP {
    inv_sa: Vec<usize>,
    spt: Vec<Vec<usize>>
}

impl LCP {
    pub fn new<T: Ord>(s: &[T], sa: &[usize]) -> LCP {
        let n = sa.len();
        assert_eq!(s.len(), n);
        let mut inv_sa = vec![0; n];
        for i in 0..n {
            inv_sa[sa[i]] = i;
        }
        let lcp = Self::create_lcp(s, sa);
        let spt = Self::create_sparse_table(&lcp);
        LCP {
            inv_sa: inv_sa,
            spt: spt,
        }
    }
    fn create_lcp<T: Ord>(s: &[T], sa: &[usize]) -> Vec<usize> {
        let n = s.len();
        let mut rank = vec![0; n];
        let mut lcp = vec![0; n - 1];
        for i in 0..n {
            rank[sa[i]] = i;
        }
        let mut h: usize = 0;
        for i in 0..n {
            if rank[i] == 0 {
                continue;
            }
            let j = sa[rank[i] - 1];
            h = h.saturating_sub(1);
            while j + h < n && i + h < n {
                if s[j + h] != s[i + h] {
                    break;
                }
                h += 1;
            }
            lcp[rank[i] - 1] = h;
        }
        return lcp;
    }
    
    fn create_sparse_table(lcp: &[usize]) -> Vec<Vec<usize>> {
        let n = lcp.len();
        let mut h: usize = 1;
        while (1 << h) <= n {
            h += 1;
        }
        let mut st: Vec<Vec<usize>> = vec![Vec::new(); h];
        st[0] = Vec::from(lcp);
        for j in 1 .. h {
            st[j] = vec![0; n + 1 - (1 << j)];
            for i in 0 .. n + 1 - (1 << j) {
                st[j][i] = std::cmp::min(
                    st[j - 1][i],
                    st[j - 1][i + 1_usize.wrapping_shl(j as u32 - 1)]);
            }
        }
        return st;
    }
    
    pub fn get_lcp(&self, f: usize, s: usize) -> usize {
        let n = self.inv_sa.len();
        if f == n || s == n {
            return 0;
        }
        let f = self.inv_sa[f];
        let s = self.inv_sa[s];
        let (f, s) = 
            if f > s {
                (s, f)
            } else {
                (f, s)
            };
        assert!(f < s);
        let usize_size = usize::max_value().count_ones();
        let diff = usize_size - 1 - (s - f).leading_zeros(); // topmost 1
        return std::cmp::min(self.spt[diff as usize][f],
                             self.spt[diff as usize][s - 1_usize.wrapping_shl(diff)]);
    }
}

/**
 * Segment Tree. This data structure is useful for fast folding on intervals of an array
 * whose elements are elements of monoid I. Note that constructing this tree requires the identity
 * element of I and the operation of I.
 * Verified by: yukicoder No. 259 (http://yukicoder.me/submissions/100581)
 *              AGC015-E (http://agc015.contest.atcoder.jp/submissions/1461001)
 *              yukicoder No. 833 (https://yukicoder.me/submissions/703521)
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
    #[allow(unused)]
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

const INF: i64 = 1 << 50;

fn solve(s: &[char], t: &[char]) -> i64 {
    let n = s.len();
    let m = t.len() / 2;
    let mut dp = vec![INF; m + 1];
    dp[m] = 0;
    let mut st = SegTree::new(m + 1, min, INF);
    st.update(m, 0);
    let sa = suffix_array_lowercase(&t);
    let lcp = LCP::new(&t, &sa);
    for i in (0..m).rev() {
        let len = min(m - i, lcp.get_lcp(i, 2 * m - i));
        if len > 0 {
            dp[i] = 1 + st.query(i + 1, i + len + 1);
        }
        st.update(i, dp[i]);
    }
    let mut ans = dp[0];
    for i in 0..n {
        if i < m && s[i] == t[i] {
            ans = min(ans, dp[i + 1]);
        } else {
            break;
        }
    }
    ans
}

fn main() {
    let _n: usize = get();
    let m: usize = get();
    let s: Vec<char> = get_word().chars().collect();
    let t: Vec<char> = get_word().chars().collect();
    if s == t {
        println!("0");
        return;
    }
    let mut revt = t.clone();
    revt.reverse();
    if m % 2 != 0 || t != revt {
        println!("-1");
        return;
    }
    let mut mi = solve(&s, &t);
    let mut s = s;
    s.reverse();
    mi = min(mi, solve(&s, &t));
    println!("{}", if mi >= INF { -1 } else { mi + 1 });
}
