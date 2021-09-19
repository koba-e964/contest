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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
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

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

const INF: i64 = 1 << 50;

// Solved with hints
// Tags: off-by-one-errors
fn main() {
    input! {
        n: usize,
        xa: [(i64, i64); n],
    }
    let mut xa = xa;
    xa.sort();
    let lo = xa.lower_bound(&(0, -1));
    let hi = xa.lower_bound(&(1, -1));
    // [0, lo), [hi, n)
    let mut dp = vec![vec![vec![[-INF; 2]; n + 1]; n + 1]; n + 1];
    for s in (0..n + 1).rev() {
        for i in 0..n - s + 1 {
            let j = i + s;
            if i > lo || j < hi {
                continue;
            }
            for k in 1..n + 1 {
                for b in 0..2 {
                    let (from, fa) = if b == 0 {
                        if i == lo {
                            (0, 0)
                        } else {
                            xa[i]
                        }
                    } else {
                        if j == hi {
                            (0, 0)
                        } else {
                            xa[j - 1]
                        }
                    };
                    let mut me = fa;
                    if j + 1 <= n {
                        let to = xa[j].0;
                        let val = dp[i][j + 1][k][1] - (from - to).abs() * k as i64;
                        me.chmax(val);
                        if k >= 1 {
                            let val = dp[i][j + 1][k - 1][1] - (from - to).abs() * (k - 1) as i64 + fa;
                            me.chmax(val);
                        }
                    }
                    if i >= 1 {
                        let to = xa[i - 1].0;
                        let val = dp[i - 1][j][k][0] - (from - to).abs() * k as i64;
                        me.chmax(val);
                        if k >= 1 {
                            let val = dp[i - 1][j][k - 1][0] - (from - to).abs() * (k - 1) as i64 + fa;
                            me.chmax(val);
                        }
                    }
                    dp[i][j][k][b] = me;
                }
            }
        }
    }
    if false {
        for i in 0..lo + 1 {
            for j in hi..n + 1 {
                eprintln!("dp[{}, {}] = {:?}", i, j, dp[i][j]);
            }
        }
    }
    let mut ma = 0;
    for k in 0..n + 1 {
        for i in 0..2 {
            ma.chmax(dp[lo][hi][k][i]);
        }
    }
    for i in lo..hi {
        ma += xa[i].1;
    }
    println!("{}", ma);
}
