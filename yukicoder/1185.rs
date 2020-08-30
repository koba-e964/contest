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

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

// ref: https://kuretchi.github.io/blog/entries/automaton-dp/
/// An (almost) DFA. trans is allowed to return None.
/// S: alphabet (the set consisting of letters)
trait DFA<S> {
    fn size(&self) -> usize;
    fn trans(&self, state: usize, char: S) -> Option<usize>;
    fn init(&self) -> Vec<usize>;
    fn is_final_state(&self, state: usize) -> bool;
}

struct Prod<A, B>(A, B);

impl<S: Copy, A: DFA<S>, B: DFA<S>> DFA<S> for Prod<A, B> {
    fn size(&self) -> usize {
        self.0.size() * self.1.size()
    }
    fn trans(&self, state: usize, char: S) -> Option<usize> {
        let w = self.1.size();
        let (x, y) = (state / w, state % w);
        if let Some(to1) = self.0.trans(x, char) {
            if let Some(to2) = self.1.trans(y, char) {
                return Some(to1 * w + to2);
            }
        }
        None
    }
    fn init(&self) -> Vec<usize> {
        let w = self.1.size();
        let b_init = self.1.init();
        let mut ans = vec![];
        for av in self.0.init() {
            for &bv in &b_init {
                ans.push(av * w + bv);
            }
        }
        ans
    }
    fn is_final_state(&self, state: usize) -> bool {
        let w = self.1.size();
        let (x, y) = (state / w, state % w);
        self.0.is_final_state(x) && self.1.is_final_state(y)
    }
}

trait ActionMonoid<S> {
    type T;
    fn add(&self, x: Self::T, y: Self::T) -> Self::T;
    fn act(&self, x: Self::T, letter: S) -> Self::T;
    fn zero(&self) -> Self::T;
    fn one(&self) -> Self::T;
}

struct Add;

impl<S> ActionMonoid<S> for Add {
    type T = i64;
    fn add(&self, x: i64, y: i64) -> i64 {
        x + y
    }
    fn act(&self, x: i64, _y: S) -> i64 {
        x
    }
    fn zero(&self) -> i64 { 0 }
    fn one(&self) -> i64 { 1 }
}

/// Digital DP.
/// Finds \sum_{s < a, s in final} f(s), \sum_{s = a, s in final} f(s).
/// Verified by: yukicoder No.1106
/// https://yukicoder.me/submissions/510954
fn digital_dp<S: Copy + Ord, A: DFA<S>, M: ActionMonoid<S>>(
    dfa: A, monoid: M,
    alpha: &[S], a: &[S]
) -> [M::T; 2] where M::T: Copy {
    let n = dfa.size();
    let len = a.len();
    let init = dfa.init();
    let mut dp = vec![vec![[monoid.zero(); 2]; n]; len + 1];
    for &v in &init {
        dp[0][v][1] = monoid.one();
    }
    for i in 0..len {
        for j in 0..n {
            for eq in 0..2 {
                let val = dp[i][j][eq];
                for &c in alpha {
                    if eq == 1 && c > a[i] {
                        continue;
                    }
                    if let Some(to) = dfa.trans(j, c) {
                        let toeq = eq & if c == a[i] { 1 } else { 0 };
                        dp[i + 1][to][toeq]
                            = monoid.add(dp[i + 1][to][toeq],
                                         monoid.act(val, c));
                    }
                }
            }
        }
    }
    let mut ans = [monoid.zero(); 2];
    for i in 0..n {
        if dfa.is_final_state(i) {
            for j in 0..2 {
                ans[j] = monoid.add(ans[j], dp[len][i][j]);
            }
        }
    }
    ans
}

struct A;

impl DFA<i64> for A {
    fn size(&self) -> usize {
        1
    }
    fn trans(&self, _state: usize, char: i64) -> Option<usize> {
        if char % 3 == 0 {
            Some(0)
        } else {
            None
        }
    }
    fn init(&self) -> Vec<usize> {
        vec![0]
    }
    fn is_final_state(&self, _state: usize) -> bool {
        true
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: i64,
    }
    let div = max(min(n, 100), 10);
    let mut tot = (div - 9) / 3;
    if div >= 100 {
        let mut dig = vec![];
        let mut v = n;
        while v > 0 {
            dig.push(v % 10);
            v /= 10;
        }
        dig.reverse();
        let alpha: Vec<_> = (0..10).collect();
        let ans = digital_dp(A, Add, &alpha, &dig);
        tot += ans[0] + ans[1] - 16;
    }
    puts!("{}\n", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
