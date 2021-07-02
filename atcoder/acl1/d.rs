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

// Tags: doubling, binary-lifting
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, k: i64,
        x: [i64; n],
        lr: [(usize1, usize)],
    }
    const B: usize = 18;
    let mut rgt = vec![vec![None; n]; B];
    let mut lft = vec![vec![None; n]; B];
    for i in 0..n {
        let idx = x.lower_bound(&(x[i] + k));
        if idx < n {
            rgt[0][i] = Some(idx);
        }
        let idx = x.upper_bound(&(x[i] - k));
        if idx > 0 {
            lft[0][i] = Some(idx - 1);
        }
    }
    for i in 0..B - 1 {
        for j in 0..n {
            if let Some(to) = rgt[i][j] {
                rgt[i + 1][j] = rgt[i][to];
            }
            if let Some(to) = lft[i][j] {
                lft[i + 1][j] = lft[i][to];
            }
        }
    }
    let mut rsum = vec![vec![0; n]; B];
    let mut lsum = vec![vec![0; n]; B];
    for i in 0..n {
        rsum[0][i] = i;
        lsum[0][i] = i;
    }
    // usize has 64 bits
    for i in 0..B - 1 {
        for j in 0..n {
            if let Some(to) = rgt[i][j] {
                rsum[i + 1][j] = rsum[i][j] + rsum[i][to];
            }
            if let Some(to) = lft[i][j] {
                lsum[i + 1][j] = lsum[i][j] + lsum[i][to];
            }
        }
    }
    for (l, r) in lr {
        let mut to_r = l;
        let mut to_l = r - 1;
        let mut c = 0;
        for i in (0..B).rev() {
            if let Some(tmp) = rgt[i][to_r] {
                if tmp < r {
                    to_r = tmp;
                    to_l = lft[i][to_l].unwrap();
                    c += 1 << i;
                }
            }
        }
        assert!(to_l >= l);
        let mut ans = 0;
        to_r = l;
        to_l = r - 1;
        for i in 0..B {
            if (c & 1 << i) != 0 {
                ans = ans + lsum[i][to_l] - rsum[i][to_r];
                to_r = rgt[i][to_r].unwrap();
                to_l = lft[i][to_l].unwrap();
            }
        }
        ans = ans + to_l - to_r + c + 1;
        puts!("{}\n", ans);
    }
}
