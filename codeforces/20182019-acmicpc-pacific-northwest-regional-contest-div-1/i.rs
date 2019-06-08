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

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

const INF: i64 = 1 << 40;

/*
 * Convex hull trick for max queries.
 * Manages multiple linear graphs.
 * Lines that are not necessary to calculate maximum values are deleted.
 * Verified by: yukicoder No.409 (http://yukicoder.me/submissions/143613)
 * https://codeforces.com/gym/101982/submission/55311987
 */
struct CHTIncr {
    dat: Vec<(i64, i64)>, // (a,b) -> y = a * x + b
    cur_idx: usize, // current index (in 0 .. dat.len())
}

impl CHTIncr {
    fn new() -> Self {
        CHTIncr { dat: Vec::new(), cur_idx: 0, }
    }
    fn check(a: (i64, i64), b: (i64, i64), c: (i64, i64)) -> bool {
        (b.0 - a.0) * (c.1 - b.1) >= (b.1 - a.1) * (c.0 - b.0)
    }
    /*
     * added.0 must be the largest.
     */
    fn add(&mut self, added: (i64, i64)) {
        while self.dat.len() >= 2 {
            let l = self.dat.len();
            if Self::check(self.dat[l - 2], self.dat[l - 1], added) {
                if self.cur_idx == self.dat.len() - 1 {
                    self.cur_idx -= 1;
                }
                self.dat.pop().unwrap();
            } else {
                break;
            }
        }
        self.dat.push(added);
    }
    #[allow(dead_code)]
    fn get(&self) -> Vec<(i64, i64)> {
        self.dat.clone()
    }
    // Returns max(line.0 * x + line.1).
    // The caller must ensure that x is non-decreasing,
    // when calls are sorted in chronological order.
    fn query(&mut self, x: i64) -> Option<i64> {
        let n = self.dat.len();
        if n == 0 {
            return None;
        }
        while self.cur_idx < n - 1 {
            let line = self.dat[self.cur_idx];
            let line2 = self.dat[self.cur_idx + 1];
            if line.0 * x + line.1 < line2.0 * x + line2.1 {
                self.cur_idx += 1;
            } else {
                break;
            }
        }
        let line = self.dat[self.cur_idx];
        Some(line.0 * x + line.1)
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
    }
    let mut inv_c: i64 = 0;
    {
        let mut cnt = vec![0; k + 1];
        for i in 0..n {
            if p[i] == 0 { continue; }
            for j in p[i] + 1..k + 1 {
                inv_c += cnt[j];
            }
            cnt[p[i]] += 1;
        }
    }
    let mut acc_0_l = vec![0; n + 1];
    for i in 0..n {
        acc_0_l[i + 1] = acc_0_l[i];
        if p[i] == 0 {
            acc_0_l[i + 1] += 1;
        }
    }
    let mut dp = vec![vec![-INF; n + 2]; 2];
    dp[0][0] = 0;
    for j in (1..k + 1).rev() {
        let mut acc_l = vec![0; n + 1];
        let mut acc_r = vec![0; n + 1];
        for i in 0..n {
            acc_l[i + 1] = acc_l[i];
            if j < p[i] {
                acc_l[i + 1] += 1;
            }
        }
        for i in (0..n).rev() {
            acc_r[i] = acc_r[i + 1];
            if j > p[i] && p[i] != 0 {
                acc_r[i] += 1;
            }
        }
        let mut inv_cv = vec![0; n + 1];
        for i in 0..n {
            inv_cv[i + 1] = inv_cv[i];
            if p[i] != 0 { continue; }
            inv_cv[i + 1] += acc_l[i];
        }
        let mut inv_vc = vec![0; n + 1];
        for i in (0..n).rev() {
            inv_vc[i] = inv_vc[i + 1];
            if p[i] != 0 { continue; }
            inv_vc[i] += acc_r[i + 1];
        }
        for i in 0..n + 1 {
            dp[1][i] = -INF;
        }
        let mut cht = CHTIncr::new();
        for i in 0..n + 1 {
            dp[1][i] = max(dp[1][i], dp[0][i]);
            if let Some(val) = cht.query(acc_0_l[i]) {
                dp[1][i] = max(dp[1][i], val + inv_cv[i] - inv_vc[i]);
            }
            cht.add((acc_0_l[i], dp[0][i]
                     - inv_cv[i] + inv_vc[i] - acc_0_l[i] * acc_0_l[i]));
        }
        dp.swap(0, 1);
    }

    puts!("{}\n", inv_c + dp[0][n]);
}

fn main() {
    solve();
}
