#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
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

/*
 * Suffix Array by Manber & Myers.
 * Verified by: AtCoder ARC050 (http://arc050.contest.atcoder.jp/submissions/818912)
 * Reference: http://mayokoex.hatenablog.com/entry/2016/04/03/145845
 */
fn create_sa(s: &[char]) -> Vec<usize> {
    let n = s.len();
    let mut sa: Vec<usize> = (0 .. n + 1).collect();
    let mut rank: Vec<usize> = vec![0; n + 1];
    let mut tmp = vec![0; n + 1];

    for i in 0 .. n + 1 {
        rank[i] = if i < n { s[i] as usize + 1 } else { 0_usize };
    }
    let mut k = 1;
    while k <= n {
        {
            let key = |i: &usize| {
                let ri = if i + k <= n { rank[i + k] as i32 } else { -1 };
                (rank[*i], ri)
            };
            sa.sort_by_key(&key);
            tmp[sa[0]] = 0;
            for i in 1 .. n + 1 {
                tmp[sa[i]] = tmp[sa[i - 1]]
                    + if key(&sa[i - 1]) < key(&sa[i]) { 1 } else { 0 };
            }
        }
        rank.clone_from_slice(&tmp);
        k *= 2;
    }
    return sa;
}

struct LCP {
    inv_sa: Vec<usize>,
    spt: Vec<Vec<usize>>
}

impl LCP {
    pub fn new(s: &[char], sa: &[usize]) -> LCP {
        let n = sa.len() - 1;
        let mut inv_sa = vec![0; n + 1];
        for i in 0 .. n + 1 {
            inv_sa[sa[i]] = i;
        }
        let lcp = Self::create_lcp(s, sa);
        let spt = Self::create_sparse_table(&lcp);
        LCP {
            inv_sa: inv_sa,
            spt: spt,
        }
    }
    fn create_lcp(s: &[char], sa: &[usize]) -> Vec<usize> {
        let n = s.len();
        let mut rank = vec![0; n + 1];
        let mut lcp = vec![0; n];
        for i in 0 .. n + 1 {
            rank[sa[i]] = i;
        }
        let mut h: usize = 0;
        lcp[0] = 0;
        for i in 0 .. n {
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

// Manacher http://snuke.hatenablog.com/entry/2014/12/02/235837
// Verified by https://atcoder.jp/contests/wupc2019/submissions/4540033
fn manacher<T: PartialEq>(tmp: &[T]) -> Vec<usize> {
    let n = tmp.len();
    let mut r = vec![0; n];
    {
        let mut i = 0;
        let mut j = 0;
        while i < n {
            while i >= j && i + j < n && tmp[i - j] == tmp[i + j] {
                j += 1;
            }
            r[i] = j;
            let mut k = 1;
            while i >= k && i + k < n && k + r[i - k] < j {
                r[i + k] = r[i - k];
                k += 1;
            }
            i += k;
            j -= k;
        }
    }
    r
}


fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    input! {
        s_orig: chars,
    }
    let n = s_orig.len();
    let mut s = vec!['.'; 2 * n - 1];
    for i in 0..n {
        s[2 * i] = s_orig[i];
    }
    let mut revs = s_orig.clone();
    revs.reverse();
    let mut srevs = s_orig.clone();
    srevs.extend_from_slice(&revs);
    // data structure
    let r = manacher(&s);
    let sa = create_sa(&srevs);
    let lcp = LCP::new(&srevs, &sa);
    let mut ma = 1;
    for i in 0..2 * n - 1 {
        let r = r[i];
        let mut curma = 0;
        let lft;
        let rgt;
        lft = (i + 1 + 1 - r) / 2;
        rgt = (i + r + 1) / 2;
        if rgt - lft == n {
            ma = max(ma, n - 1);
            continue;
        }
        let base = rgt - lft;
        // delete s_orig[rgt]
        // Want to compare s_orig[0..lft].reverse() and s_orig[rgt + 1..n]
        if rgt < n {
            let lcp_len = lcp.get_lcp(rgt + 1, 2 * n - lft);
            let total_len = base + 2 * min(lcp_len, n - rgt - 1);
            curma = max(curma, total_len);
        }
        // delete s_orig[lft - 1]
        // Want to compare s_orig[0..lft - 1].reverse() and s_orig[rgt..n]
        if lft > 0 {
            let lcp_len = lcp.get_lcp(rgt, 2 * n - lft + 1);
            let total_len = base + 2 * min(lcp_len, n - rgt);
            curma = max(curma, total_len);
        }
        ma = max(ma, curma);
    }
    puts!("{}\n", ma);
}

fn main() {
    solve();
}
