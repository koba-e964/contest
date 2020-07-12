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
