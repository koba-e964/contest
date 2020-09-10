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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

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

fn sa_is_i32<T: Threshold>(s: &[i32], upper: i32) -> Vec<usize> {
    let s: Vec<usize> = s.iter().map(|&x| x as usize).collect();
    sa_is::<T>(&s, upper as usize)
}

pub fn suffix_array_manual(s: &[i32], upper: i32) -> Vec<usize> {
    assert!(upper >= 0);
    for &elem in s {
        assert!(0 <= elem && elem <= upper);
    }
    sa_is_i32::<DefaultThreshold>(s, upper)
}

pub fn suffix_array_arbitrary<T: Ord>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by_key(|&i| &s[i]);
    let mut s2 = vec![0; n];
    let mut now = 0;
    for i in 0..n {
        if i > 0 && s[idx[i - 1]] != s[idx[i]] {
            now += 1;
        }
        s2[idx[i]] = now;
    }
    sa_is_i32::<DefaultThreshold>(&s2, now)
}

pub fn suffix_array(s: &str) -> Vec<usize> {
    let s2: Vec<usize> = s.bytes().map(|x| x as usize).collect();
    sa_is::<DefaultThreshold>(&s2, 255)
}

// Reference:
// T. Kasai, G. Lee, H. Arimura, S. Arikawa, and K. Park,
// Linear-Time Longest-Common-Prefix Computation in Suffix Arrays and Its
// Applications
pub fn lcp_array_arbitrary<T: Ord>(s: &[T], sa: &[usize]) -> Vec<usize> {
    let n = s.len();
    assert!(n >= 1);
    let mut rnk = vec![0; n];
    for i in 0..n {
        rnk[sa[i]] = i;
    }
    let mut lcp = vec![0; n - 1];
    let mut h = 0;
    for i in 0..n - 1 {
        if h > 0 {
            h -= 1;
        }
        if rnk[i] == 0 {
            continue;
        }
        let j = sa[rnk[i] - 1];
        while j + h < n && i + h < n {
            if s[j + h] != s[i + h] {
                break;
            }
            h += 1;
        }
        lcp[rnk[i] - 1] = h;
    }
    lcp
}

pub fn lcp_array(s: &str, sa: &[usize]) -> Vec<usize> {
    let s: &[u8] = s.as_bytes();
    lcp_array_arbitrary(s, sa)
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input!(s: chars);
    let s: String = s.into_iter().collect();
    let sa = suffix_array(&s);
    let lcp = lcp_array(&s, &sa);
    let mut tot: i64 = 0;
    tot += (s.len() - sa[0]) as i64;
    for i in 1..s.len() {
        tot -= lcp[i - 1] as i64;
        tot += (s.len() - sa[i]) as i64;
    }
    puts!("{}\n", tot);
}
