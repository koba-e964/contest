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

const B: usize = 17;

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize, q: usize,
        abst: [(usize1, usize1, i64, i64); m],
        xyz: [(i64, usize1, i64); q],
    }
    let mut coord = vec![vec![]; n];
    for &(a, _, s, _) in &abst {
        coord[a].push(2 * s + 1);
    }
    for i in 0..n {
        coord[i].sort_unstable(); coord[i].dedup();
    }
    let mut to = vec![vec![]; n];
    let mut to_exact = vec![vec![]; n];
    for i in 0..n {
        let l = coord[i].len();
        to[i] = vec![(0, 0); l];
        to_exact[i] = vec![(0, 0); l];
    }
    for &(a, b, s, t) in &abst {
        let idx = coord[a].binary_search(&(2 * s + 1)).unwrap();
        let tidx = coord[b].lower_bound(&(2 * t + 1));
        to[a][idx] = (b, tidx);
        to_exact[a][idx] = (b, 2 * t + 1);
    }
    let mut dp = vec![vec![vec![]; n]; B];
    for i in 0..n {
        let l = coord[i].len();
        dp[0][i] = vec![(0, 0); l + 1];
        for j in 0..l {
            dp[0][i][j] = to[i][j];
        }
        dp[0][i][l] = (i, l);
    }
    for i in 0..B - 1 {
        for j in 0..n {
            let l = coord[j].len();
            dp[i + 1][j] = vec![(0, 0); l + 1];
            for k in 0..l + 1 {
                let (x, y) = dp[i][j][k];
                dp[i + 1][j][k] = dp[i][x][y];
            }
        }
    }
    for &(x, y, z) in &xyz {
        let mut a = y;
        let mut b = coord[a].lower_bound(&(2 * x));
        for i in (0..B).rev() {
            let (c, d) = dp[i][a][b];
            if d < coord[c].len() && coord[c][d] < 2 * z {
                a = c;
                b = d;
            }
        }
        // Use the bus or not?
        if b >= to_exact[a].len() {
            puts!("{}\n", a + 1);
        } else {
            let (to, time) = to_exact[a][b];
            if time <= 2 * z {
                puts!("{}\n", to + 1);
            } else if coord[a][b] >= 2 * z {
                puts!("{}\n", a + 1);
            } else {
                puts!("{} {}\n", a + 1, to + 1);
            }
        }
    }
}
