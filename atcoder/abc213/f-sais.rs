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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600 * 5; // 500 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

// https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/string.rs
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
    fn create_lcp<T: Ord>(s: &[T], sa: &[usize]) -> Vec<usize> {
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

fn dfs(l: usize, r: usize, bias: usize,
       sa: &[usize], lcp: &LCP, ans: &mut [i64]) {
    if l >= r {
        return;
    }
    if l + 1 == r {
        let val = (sa.len() - 1 - sa[l] - bias) as i64;
        ans[l] += val;
        ans[r] -= val;
        return;
    }
    let mut pass = l + 1;
    let mut fail = r + 1;
    while fail - pass > 1 {
        let mid = (fail + pass) / 2;
        if lcp.get_lcp(sa[l], sa[mid - 1]) > bias {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    let newbias = if l + 1 == pass {
        sa.len() - 1 - sa[l]
    } else {
        lcp.get_lcp(sa[l], sa[pass - 1])
    };
    let val = (newbias - bias) as i64 * (pass - l) as i64;
    // eprintln!("range add: {} {} {} (bias: {} => {})", l, pass, val, bias, newbias);
    ans[l] += val;
    ans[pass] -= val;
    dfs(l, pass, newbias, sa, lcp, ans);
    dfs(pass, r, bias, sa, lcp, ans);
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        s: chars,
    }
    let mut sa = suffix_array_lowercase(&s);
    sa.insert(0, n);
    let lcp = LCP::new(&s, &sa);
    let mut ans = vec![0i64; n + 2];
    dfs(1, n + 1, 0, &sa, &lcp, &mut ans);
    for i in 0..n {
        ans[i + 1] += ans[i];
    }
    for i in 0..n {
        puts!("{}\n", ans[lcp.inv_sa[i]]);
    }
}
