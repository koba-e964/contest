// Handles minimum cost circulation problems.
// ref: https://atcoder.jp/contests/abc231/submissions/27857324
// ref: https://gist.github.com/brunodccarvalho/fb9f2b47d7f8469d209506b336013473
// ref: https://people.orie.cornell.edu/dpw/orie633/LectureNotes/lecture11.pdf
// Depends on: graph/MinCostFlow.rs
// Verified by:
// - https://atcoder.jp/contests/practice2/submissions/70013984
// - https://atcoder.jp/contests/practice2/submissions/70014032
pub struct MinCostCirc {
    mcf: MinCostFlow,
    sup: Vec<Cap>,
    cost: Cost,
    edges: Vec<(usize, usize, Cap)>, // (u, index in graph[u], cap)
}

impl MinCostCirc {
    pub fn new(n: usize) -> Self {
        let mcf = MinCostFlow::new(2 + n);
        MinCostCirc {
            mcf: mcf,
            sup: vec![0; n],
            cost: 0,
            edges: vec![],
        }
    }
    pub fn add_edge(&mut self, a: usize, b: usize, (dem, cap): (Cap, Cap), cost: Cost) {
        assert!(dem <= cap);
        if cost >= 0 {
            self.add_edge_inner(a, b, (dem, cap), cost, false);
        } else {
            self.add_edge_inner(b, a, (-cap, -dem), -cost, true);
        }
    }
    fn add_edge_inner(
        &mut self, a: usize, b: usize,
        (dem, cap): (Cap, Cap), cost: Cost, flipped: bool,
    ) {
        assert!(dem <= cap);
        assert!(cost >= 0);
        self.cost += dem as Cost * cost;
        let index = self.mcf.graph[2 + if flipped { b } else { a }].len();
        self.mcf.add_edge(2 + a, 2 + b, cap - dem, cost);
        self.sup[b] += dem;
        self.sup[a] -= dem;
        if flipped {
            // Let e' = (a -> b) and e = (b -> a). (Note that the original edge was b -> a.)
            // If after min_cost() e.cap = x holds,
            // the actual flow on the edge e is cap - x = cap - e.cap
            self.edges.push((b, index, -dem));
        } else {
            // If after min_cost() e.cap = cap - dem - x holds,
            // the actual flow is dem + x = cap - e.cap
            self.edges.push((a, index, cap));
        }
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
            Some(cost + self.cost)
        }
    }
    // Call it only after calling min_cost.
    #[allow(unused)]
    pub fn sol(&self) -> Vec<(usize, usize, Cap)> {
        let mut ans = vec![];
        for &(i, j, cap) in &self.edges {
            let e = &self.mcf.graph[2 + i][j];
            let amount = cap - e.cap;
            if amount != 0 && e.to >= 2 {
                ans.push((i, e.to - 2, amount));
            }
        }
        ans
    }
}
