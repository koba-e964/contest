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

// https://yukicoder.me/problems/no/1391 (3)
// x が決まると B は決まる。
// |x - A_i| 同士の大小が変わりうるのは (A_i + A_j) / 2 の形の点に限られる。
// 目的関数は連続関数で左端での傾きは N-2K、右端での傾きは 2K-N であるため、傾きの変化の回数は 2|2K-N| 以下。
// -> これは誤り。A = [-1, 1], K = 1 のとき目的関数は -2 (|x| > 1), -2|x| (|x| <= 1) であり、左端と右端での傾きは正しいが傾きの変化は 2 回で正しくない。
// 特定の x に対する min_B f_B(x) の評価は累積和や二分探索を使うことで O(log N log A)-time/query でできる。
// A_i と A_{i+1} の間で傾きが増えないので、最小値は両端のどちらかで実現される。これを使えば A_i すべてにおける値の最小値が答えであることが示せる。
fn main() {
    input! {
        n: usize, k: usize,
        a: [i64; n],
    }
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + a[i];
    }
    let f = |i: usize| {
        let mut pass = 1i64 << 31;
        let mut fail = -1;
        let x = a[i];
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            let over = i - a[..i].lower_bound(&(x - mid))
                + a[i..].upper_bound(&(x + mid));
            if over >= k {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        let lo = a[..i].lower_bound(&(x - pass));
        let hi = a[i..].upper_bound(&(x + pass)) + i;
        let g = |s: usize, t: usize| acc[t] - acc[s] - (t - s) as i64 * x;
        -g(hi, n) + g(i, hi) - g(lo, i) + g(0, lo)
    };
    let mut ans = 1 << 60;
    for i in 0..n {
        ans = min(ans, f(i));
    }
    println!("{}", ans);
}
