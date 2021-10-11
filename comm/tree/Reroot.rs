// Verified by: https://yukicoder.me/submissions/706785
trait LeaveOne: Default + Clone {
    type T: Default + Clone;
    fn build(vals: &[Self::T]) -> Self;
    fn leave_one(&self, excl: Self::T) -> Self::T;
    fn exchange_one(&self, excl: Self::T, incl: Self::T) -> Self::T;
    fn add_one(&self, incl: Self::T) -> Self::T;
    fn as_is(&self) -> Self::T;
}

struct Reroot<LOO: LeaveOne> {
    #[allow(unused)]
    pub dp1: Vec<LOO::T>,
    #[allow(unused)]
    pub dp2: Vec<Vec<LOO::T>>,
    #[allow(unused)]
    pub dp_loo: Vec<LOO>,
}

impl<LOO: LeaveOne> Reroot<LOO> {
    pub fn new(g: &[Vec<usize>]) -> Self {
        let n = g.len();
        let mut dp1 = vec![LOO::T::default(); n];
        let mut dp2 = vec![vec![]; n];
        let mut dp_loo = vec![LOO::default(); n];
        Self::dfs1(0, n, &g, &mut dp_loo, &mut dp2);
        Self::dfs2(0, n, &g, &mut dp1, &dp_loo, &mut dp2, LOO::T::default());
        Reroot {
            dp1: dp1,
            dp2: dp2,
            dp_loo: dp_loo,
        }
    }
    fn dfs1(
        v: usize, par: usize, g: &[Vec<usize>],
        dp_loo: &mut [LOO], dp2: &mut [Vec<LOO::T>],
    ) {
        let mut mydp2 = vec![LOO::T::default(); g[v].len()];
        let mut chval = vec![];
        for i in 0..g[v].len() {
            let w = g[v][i];
            if w == par { continue; }
            Self::dfs1(w, v, g, dp_loo, dp2);
            mydp2[i] = dp_loo[w].as_is();
            chval.push(mydp2[i].clone());
        }
        dp_loo[v] = LOO::build(&chval);
        dp2[v] = mydp2;
    }
    fn dfs2(
        v: usize, par: usize, g: &[Vec<usize>],
        dp1: &mut [LOO::T],
        dp_loo: &[LOO],
        dp2: &mut [Vec<LOO::T>],
        passed: LOO::T,
    ) {
        for i in 0..g[v].len() {
            let w = g[v][i];
            if w == par {
                dp2[v][i] = passed.clone();
                continue;
            }
            let inherited = if par >= g.len() {
                dp_loo[v].leave_one(dp2[v][i].clone())
            } else {
                dp_loo[v].exchange_one(dp2[v][i].clone(), passed.clone())
            };
            Self::dfs2(w, v, g, dp1, dp_loo, dp2, inherited);
        }
        dp1[v] = if par >= g.len() {
            dp_loo[v].as_is()
        } else {
            dp_loo[v].add_one(passed)
        };
    }
}
