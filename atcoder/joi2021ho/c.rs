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

/// Binary Indexed Tree (Fenwick Tree). Holds an array of type T.
/// T is a commutative monoid. Indices are 1 .. n.
/// Verified by yukicoder No.404 (http://yukicoder.me/submissions/155373)
struct BIT<T> {
    n: usize,
    ary: Vec<T>,
    e: T,
}

impl<T: Clone + std::ops::AddAssign<T>> BIT<T> {
    fn new(n: usize, e: T) -> Self {
        let n = n.next_power_of_two();
        BIT { n: n, ary: vec![e.clone(); n + 1], e: e }
    }
    /**
     * gets the sum in [1 .. idx]
     * @param idx
     * @return sum
     */
    fn accum(&self, mut idx: usize) -> T {
        let mut sum = self.e.clone();
        while idx > 0 {
            sum += self.ary[idx].clone();
            idx &= idx - 1;
        }
        sum
    }
    /**
     * performs data[idx] += val;
     */
    fn add<U: Clone>(&mut self, mut idx: usize, val: U)
        where T: std::ops::AddAssign<U> {
        assert!(idx > 0);
        let n = self.n;
        while idx <= n {
            self.ary[idx] += val.clone();
            idx += idx & idx.wrapping_neg();
        }
    }
    /// Make sure that 1 <= idx <= n.
    #[allow(unused)]
    fn single(&self, idx: usize) -> T
        where T: std::ops::Sub<Output = T> {
        self.accum(idx) - self.accum(idx - 1)
    }
}

// Tags: bubble-sort, inversion-number, dp
fn solve() {
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
        h: [usize1; n],
    }
    let mut p = vec![0; n];
    for i in 0..n {
        p[h[i]] = i;
    }
    // inv of p[i..j]
    let mut inv = vec![vec![0i64; n + 1]; n + 1];
    for i in 0..n {
        let mut bit = BIT::new(n, 0i64);
        for j in i..n {
            inv[i][j + 1] = inv[i][j] + (j - i) as i64 - bit.accum(p[j] + 1);
            bit.add(p[j] + 1, 1);
        }
    }
    let mut dp = vec![1i64 << 50; n + 1];
    dp[0] = 0;
    for i in 0..n {
        let val = dp[i];
        for j in i + 1..n + 1 {
            let x = (j - i) as i64;
            dp[j].chmin(val + inv[0][j] - inv[0][i] - 2 * inv[i][j] + x * (x - 1) / 2);
        }
    }
    puts!("{}\n", dp[n]);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
