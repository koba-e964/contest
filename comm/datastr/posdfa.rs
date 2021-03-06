// ref: https://kuretchi.github.io/blog/entries/automaton-dp/
/// An (almost) positional DFA. "Positional" means transition may change
/// depending on how many chars it has already consumed.
/// trans is allowed to return None.
/// S: alphabet (the set consisting of letters)
trait PosDFA<S> {
    /// size should remain constant whatever its position is.
    fn size(&self) -> usize;
    fn trans(&self, pos: usize, state: usize, char: S) -> Option<usize>;
    fn init(&self) -> Vec<usize>;
    fn is_final_state(&self, pos: usize, state: usize) -> bool;
}

struct Prod<A, B>(A, B);

impl<S: Copy, A: PosDFA<S>, B: PosDFA<S>> PosDFA<S> for Prod<A, B> {
    fn size(&self) -> usize {
        self.0.size() * self.1.size()
    }
    fn trans(&self, pos: usize, state: usize, char: S) -> Option<usize> {
        let w = self.1.size();
        let (x, y) = (state / w, state % w);
        if let Some(to1) = self.0.trans(pos, x, char) {
            if let Some(to2) = self.1.trans(pos, y, char) {
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
    fn is_final_state(&self, pos: usize, state: usize) -> bool {
        let w = self.1.size();
        let (x, y) = (state / w, state % w);
        self.0.is_final_state(pos, x) && self.1.is_final_state(pos, y)
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

impl PosDFA<i32> for GtZero {
    fn size(&self) -> usize {
        2
    }
    fn trans(&self, _: usize, zero: usize, c: i32) -> Option<usize> {
        if zero == 1 && c < 0 {
            return None;
        }
        Some(zero & if c == 0 { 1 } else { 0 })
    }
    fn init(&self) -> Vec<usize> {
        vec![1]
    }
    fn is_final_state(&self, _: usize, state: usize) -> bool {
        state == 0
    }
}

struct DeltaZero(usize);

impl PosDFA<i32> for DeltaZero {
    fn size(&self) -> usize {
        2 * self.0
    }
    fn trans(&self, _: usize, delta: usize, c: i32) -> Option<usize> {
        let delta = delta as i32;
        if delta + c < 0 || delta + c >= 2 * self.0 as i32 {
            return None;
        }
        Some((delta + c) as usize)
    }
    fn init(&self) -> Vec<usize> {
        vec![self.0]
    }
    fn is_final_state(&self, _: usize, state: usize) -> bool {
        state == self.0
    }
}

struct Lt(Vec<i32>);

impl PosDFA<i32> for Lt {
    fn size(&self) -> usize {
        2
    }
    fn trans(&self, pos: usize, eq: usize, c: i32) -> Option<usize> {
        if eq == 1 && c > self.0[pos] {
            return None;
        }
        Some(eq & if c == self.0[pos] { 1 } else { 0 })
    }
    fn init(&self) -> Vec<usize> {
        vec![1]
    }
    fn is_final_state(&self, _pos: usize, _state: usize) -> bool {
        true
    }
}

/// DP whose state space is DFA * position.
/// Verified by: yukicoder No.1106
/// https://yukicoder.me/submissions/510962
fn posdfa_dp<S: Copy + Ord, A: PosDFA<S>, M: ActionMonoid<S>>(
    dfa: A, monoid: M,
    alpha: &[S], len: usize,
) -> M::T where M::T: Copy {
    let n = dfa.size();
    let init = dfa.init();
    let mut dp = vec![vec![monoid.zero(); n]; len + 1];
    for &v in &init {
        dp[0][v] = monoid.one();
    }
    for i in 0..len {
        for j in 0..n {
            let val = dp[i][j];
            for &c in alpha {
                if let Some(to) = dfa.trans(i, j, c) {
                    dp[i + 1][to]
                        = monoid.add(dp[i + 1][to],
                                     monoid.act(val, c));
                }
            }
        }
    }
    let mut ans = monoid.zero();
    for i in 0..n {
        if dfa.is_final_state(len, i) {
            ans = monoid.add(ans, dp[len][i]);
        }
    }
    ans
}
