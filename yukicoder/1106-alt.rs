#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
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

struct GtZero;

impl DFA<i32> for GtZero {
    fn size(&self) -> usize {
        2
    }
    fn trans(&self, zero: usize, c: i32) -> Option<usize> {
        if zero == 1 && c < 0 {
            return None;
        }
        Some(zero & if c == 0 { 1 } else { 0 })
    }
    fn init(&self) -> Vec<usize> {
        vec![1]
    }
    fn is_final_state(&self, state: usize) -> bool {
        state == 0
    }
}

struct DeltaZero(usize);

impl DFA<i32> for DeltaZero {
    fn size(&self) -> usize {
        2 * self.0
    }
    fn trans(&self, delta: usize, c: i32) -> Option<usize> {
        let delta = delta as i32;
        if delta + c < 0 || delta + c >= 2 * self.0 as i32 {
            return None;
        }
        Some((delta + c) as usize)
    }
    fn init(&self) -> Vec<usize> {
        vec![self.0]
    }
    fn is_final_state(&self, state: usize) -> bool {
        state == self.0
    }
}

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

fn main() {
    input!(n: i64);
    const N: usize = 30;
    let alpha = [-2, -1, 0, 1, 2];
    let mut dig = vec![0; N];
    let mut v = n;
    for i in 0..N {
        let mut r = v % 5;
        if r >= 3 { r -= 5; }
        dig[N - 1 - i] = r as i32;
        v = (v - r) / 5;
    }
    let dp = digital_dp(Prod(GtZero, DeltaZero(2 * N)),
                    Add, &alpha, &dig);
    println!("{}", dp[0] + dp[1]);
}
