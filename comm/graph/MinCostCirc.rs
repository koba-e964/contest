// Handles minimum cost circulation problems.
// ref: https://atcoder.jp/contests/abc231/submissions/27857324
// ref: https://gist.github.com/brunodccarvalho/fb9f2b47d7f8469d209506b336013473
// ref: https://people.orie.cornell.edu/dpw/orie633/LectureNotes/lecture11.pdf
// Depends on: graph/MinCostFlow.rs
// Verified by: https://atcoder.jp/contests/abc231/submissions/27885174
pub struct MinCostCirc {
    mcf: MinCostFlow,
    sup: Vec<Cap>,
}

impl MinCostCirc {
    pub fn new(n: usize) -> Self {
        let mcf = MinCostFlow::new(2 + n);
        MinCostCirc {
            mcf: mcf,
            sup: vec![0; n],
        }
    }
    pub fn add_edge(&mut self, a: usize, b: usize, (dem, cap): (Cap, Cap), cost: Cost) {
        assert!(dem <= cap);
        self.mcf.add_edge(2 + a, 2 + b, cap - dem, cost);
        self.sup[b] += dem;
        self.sup[a] -= dem;
    }
    pub fn min_cost(&mut self) -> Option<Cost> {
        let s: Cap = self.sup.iter().sum();
        if s != 0 {
            return None;
        }
        let n = self.sup.len();
        let sup = &self.sup;
        let mut f = 0;
        for i in 0..n {
            if sup[i] > 0 {
                self.mcf.add_edge(0, 2 + i, sup[i], 0);
                f += sup[i];
            }
            if sup[i] < 0 {
                self.mcf.add_edge(2 + i, 1, -sup[i], 0);
            }
        }
        let cost = self.mcf.min_cost_flow(0, 1, f);
        if cost < 0 {
            None
        } else {
            Some(cost)
        }
    }
}
