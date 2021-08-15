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

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn calc(h: &[i64]) -> Vec<i64> {
    let n = h.len();
    let mut st = vec![];
    let mut ans = vec![0; n];
    let mut rgt = vec![0; n];
    for i in 0..n {
        while let Some((x, idx)) = st.pop() {
            if x >= h[i] {
                st.push((x, idx));
                break;
            }
            rgt[idx] = i;
        }
        st.push((h[i], i));
    }
    for (_, idx) in st.drain(..) {
        rgt[idx] = n;
    }
    for i in (0..n).rev() {
        // When does (h[i], i) disappear?
        // <===> Find j s.t. h[i] < max h[j..2 * j - i + 1]?
        while let Some((x, idx)) = st.pop() {
            if x >= h[i] {
                st.push((x, idx));
                break;
            }
        }
        let mut pass = 0;
        let mut fail = st.len() + 1;
        while fail - pass > 1 {
            let mid = (fail + pass) / 2;
            if st[mid - 1].0 > h[i] {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        if pass > 0 {
            let lim = (st[pass - 1].1 + i + 1) / 2;
            rgt[i] = max(i + 1, min(rgt[i], lim));
        }
        st.push((h[i], i));
    }
    for i in 0..n {
        ans[i] += 1;
        if rgt[i] < n {
            ans[rgt[i]] -= 1;
        }
    }
    for i in 0..n - 1 {
        ans[i + 1] += ans[i];
    }
    for i in 0..n {
        ans[i] -= 1;
    }
    ans
}

// Tags: stack, off-by-one-error
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize,
        h: [i64; n],
    }
    let mut ans = calc(&h);
    let mut h = h;
    h.reverse();
    let sub = calc(&h);
    for i in 0..n {
        ans[i] += sub[n - 1 - i];
    }
    putvec!(ans);
}
