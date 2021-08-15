use std::cmp::*;
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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Ref: https://judge.yosupo.jp/submission/19062
struct TwoBITs<T, F> {
    a: Vec<T>,
    bit1: Vec<T>,
    bit2: Vec<T>,
    f: F,
}

#[inline]
fn lsb(x: usize) -> usize {
    x & x.wrapping_neg()
}

impl<T: Copy + Default, F: Fn(T, T) -> T> TwoBITs<T, F> {
    fn new(a: &[T], f: F) -> Self {
        let n = a.len();
        let mut bit1 = vec![T::default(); n + 1];
        let mut bit2 = vec![T::default(); n + 1];
        bit1[1..].copy_from_slice(&a);
        bit2[1..].copy_from_slice(&a);
        for i in 1..n + 1 {
            let b = lsb(i);
            if i + b <= n {
                bit1[i + b] = f(bit1[i + b], bit1[i]);
            }
        }
        for i in (1..n + 1).rev() {
            let b = lsb(i);
            bit2[i - b] = f(bit2[i - b], bit2[i]);
        }
        TwoBITs {
            a: a.to_vec(),
            bit1: bit1,
            bit2: bit2,
            f: f,
        }
    }
    fn query(&self, l: usize, r: usize) -> T {
        if l == r {
            return self.a[l];
        }
        unsafe {
            let l = l + 1;
            let mut ans = (self.f)(
                *self.a.get_unchecked(l - 1),
                *self.a.get_unchecked(r - 1),
            );
            let mut x = l;
            while x + lsb(x) <= r + 1 {
                ans = (self.f)(ans, *self.bit2.get_unchecked(x));
                x += lsb(x);
            }
            let mut y = r;
            while y != 0 && y - lsb(y) + 1 >= l {
                ans = (self.f)(ans, *self.bit1.get_unchecked(y));
                y &= y - 1;
            }
            if x > r {
                ans
            } else {
                (self.f)(ans, *self.a.get_unchecked(x - 1))
            }
        }
    }
}

const INF: i64 = 1 << 60;

// Complexity: O(n^2 log n)
// Tags: range-min-query, monge-dp, monotone-minima
fn main() {
    // !!! range-max is not monge!!!
    // But range-min is monge. We could use this to find max costs, but we don't.
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        a: [i64; n],
    }
    let spta = TwoBITs::new(&a, max);
    let mut lft = vec![0; n];
    for j in 0..n {
        let mut pass = j + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if spta.query(mid - 1, j + 1) <= a[j] {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        lft[j] = pass - 1;
    }
    let mut dp = vec![vec![-INF; n + 1]; 2];
    let mut dp_mi = vec![vec![INF; n + 1]; 2];
    dp_mi[0][0] = 0;
    dp[0][0] = 0;
    for i in 0..n {
        {
            let (frm, lat) = dp.split_at_mut(1);
            let frm = &frm[0];
            let frm = TwoBITs::new(&frm[i..], max);
            let lat = &mut lat[0][i..];
            lat[0] = -INF;
            for j in i..n {
                let mut tmp = frm.query(max(i, lft[j]) - i, j + 1 - i) + a[j];
                if lft[j] >= i {
                    tmp = max(tmp, lat[lft[j] - i]);
                }
                lat[j - i + 1] = tmp;
            }
        }
        dp.swap(0, 1);
        {
            let (frm, lat) = dp_mi.split_at_mut(1);
            let frm = &frm[0];
            let frm = TwoBITs::new(&frm[i..], min);
            let lat = &mut lat[0][i..];
            lat[0] = INF;
            for j in i..n {
                let mut tmp = frm.query(max(i, lft[j]) - i, j + 1 - i) + a[j];
                if lft[j] >= i {
                    tmp = min(tmp, lat[lft[j] - i]);
                }
                lat[j + 1 - i] = tmp;
            }
        }
        dp_mi.swap(0, 1);
        puts!("{} {}\n", dp_mi[0][n], dp[0][n]);
    }
}
