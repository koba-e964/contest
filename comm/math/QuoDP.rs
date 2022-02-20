// DP table indexed by {floor(n / i) | 1 <= i <= n}.
// Verified by: https://atcoder.jp/contests/abc239/submissions/29553511
type K = i64;
type V = i64;
struct QuoDP {
    // stores dp[n], dp[n/2], ..., dp[n/b].
    dp_big: Vec<V>,
    dp: Vec<V>,
    n: K,
    b: K,
}

impl QuoDP {
    pub fn new(n: K, b: K) -> Self {
        let dp_big = vec![0.into(); b as usize + 1];
        let dp = vec![0.into(); (n / b) as usize];
        QuoDP {
            dp_big: dp_big,
            dp: dp,
            n: n,
            b: b,
        }
    }
    #[allow(unused)]
    pub fn keys(&self) -> Vec<K> {
        (1..self.n / self.b).chain((1..=self.b).rev().map(|x| self.n / x)).collect()
    }
    // pos should be of form floor(n / ???).
    pub fn upd<F>(&mut self, pos: K, f: F) where F: Fn(V) -> V {
        if pos >= self.n / self.b {
            let idx = self.n / pos;
            debug_assert_eq!(pos, self.n / idx);
            self.dp_big[idx as usize] = f(self.dp_big[idx as usize]);
            return;
        }
        let idx = pos as usize;
        self.dp[idx] = f(self.dp[idx]);
    }
    pub fn get(&self, pos: K) -> V {
        if pos >= self.n / self.b {
            let idx = self.n / pos;
            debug_assert_eq!(pos, self.n / idx);
            return self.dp_big[idx as usize];
        }
        let idx = pos as usize;
        self.dp[idx]
    }
    #[allow(unused)]
    pub fn init<F>(&mut self, f: F) where F: Fn(K) -> V {
        self.upd_all(|k, _| f(k));
    }
    pub fn upd_all<F>(&mut self, f: F) where F: Fn(K, V) -> V {
        for i in 0..self.dp.len() {
            self.dp[i] = f(i as K, self.dp[i]);
        }
        for i in (1..self.dp_big.len()).rev() {
            self.dp_big[i] = f(self.n / i as K, self.dp_big[i]);
        }
    }
}

impl std::fmt::Debug for QuoDP {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.dp.len() {
            writeln!(f, "{}: {}", i, self.dp[i])?;
        }
        for i in (1..self.dp_big.len()).rev() {
            writeln!(f, "{}: {}", self.n / i as K, self.dp_big[i])?;
        }
        Ok(())
    }
}
