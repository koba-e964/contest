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
    ($next:expr, ) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, [graph1; $len:expr]) => {{
        let mut g = vec![vec![]; $len];
        let ab = read_value!($next, [(usize1, usize1)]);
        for (a, b) in ab {
            g[a].push(b);
            g[b].push(a);
        }
        g
    }};
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

trait Change {
    fn chmax(&mut self, x: Self);
    fn chmin(&mut self, x: Self);
}
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) {
        if *self < x { *self = x; }
    }
    fn chmin(&mut self, x: T) {
        if *self > x { *self = x; }
    }
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

// Tags: doubling
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        d: usize, l: i64, n: usize,
        c: [usize; d],
        kft: [(usize, usize1, i64); n],
    }
    const W: usize = 100_010;
    const B: usize = 30;
    // (where to, how many times excluding now)
    let mut dp = vec![vec![(0, 0); d]; B];
    let mut occ: Vec<Vec<usize>> = vec![vec![]; W];
    for i in 0..2 * d {
        occ[c[i % d]].push(i);
    }
    let get_nxt = |i: usize, kind: usize| {
        let nxt = occ[kind].upper_bound(&i);
        let mut nxt = occ[kind][nxt] as i64;
        let q = (nxt - i as i64 + l - 1) / l;
        nxt %= d as i64;
        (nxt as usize, q)
    };
    for i in 0..d {
        dp[0][i] = get_nxt(i, c[i]);
    }
    for j in 1..B {
        for i in 0..d {
            let (y, a) = dp[j - 1][i];
            let (w, b) = dp[j - 1][y];
            dp[j][i] = (w, a + b);
        }
    }
    for j in 0..n {
        let (k, f, t) = kft[j];
        if occ[k].is_empty() {
            puts!("0\n");
            continue;
        }
        let nxt = occ[k].lower_bound(&f);
        let nxt = occ[k][nxt] as i64;
        let dist = nxt - f as i64;
        let dist = (dist + l - 1) / l;
        if dist > t - 1 {
            puts!("{}\n", 0);
            continue;
        }
        let mut ans = 1;
        let mut cur = nxt as usize % d;
        assert_eq!(c[cur], k);
        let mut rem = t - dist - 1;
        for i in (0..B).rev() {
            assert_eq!(c[cur], k);
            let (x, y) = dp[i][cur];
            if rem >= y {
                cur = x;
                ans += 1 << i;
                rem -= y;
            }
        }
        puts!("{}\n", ans);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
