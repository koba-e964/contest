use std::collections::*;
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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
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

fn calc(a: &[(usize, usize)]) -> Vec<usize> {
    let n = a.len();
    let mut ans = vec![0; n];
    if n <= 450 {
        let mut seen = HashSet::new();
        for i in 0..n {
            let v = a[i].0;
            let mut mex = v;
            while seen.contains(&mex) {
                mex += v;
            }
            ans[i] = mex;
            seen.insert(mex);
        }
        return ans;
    }
    const W: usize = 200_001;
    let mut memo = vec![0; W];
    let mut seen = vec![false; W];
    for i in 0..n {
        let v = a[i].0;
        if memo[v] < v {
            memo[v] = v;
        }
        while seen[memo[v]] {
            memo[v] += v;
        }
        ans[i] = memo[v];
        seen[memo[v]] = true;
    }
    ans
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize,
        s: [chars; n],
    }
    let mut ans = vec![0; n];
    let mut hm = HashMap::new();
    for i in 0..n {
        let m = s[i].len();
        let z = z_algorithm(&s[i]);
        let mut len = m;
        for i in 1..m {
            if z[i] >= m - i && m % i == 0 {
                len = i;
                break;
            }
        }
        let base = s[i][..len].to_vec();
        let count = (m / len) as _;
        hm.entry(base).or_insert(vec![]).push((count, i));
    }
    for (k, v) in hm {
        let res = calc(&v);
        for i in 0..v.len() {
            ans[v[i].1] = res[i] / v[i].0;
        }
    }
    putvec!(ans);
}
