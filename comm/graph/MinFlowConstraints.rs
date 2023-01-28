// Solves a flow feasibility problem with minimum flow constraints.
// Depends on: graph/Dinic.rs
// https://atcoder.jp/contests/abc285/editorial/5500
// Verified by: ABC285-G (https://atcoder.jp/contests/abc285/submissions/38362562)
pub struct MinFlowConstraints(Dinic<i32>);

impl MinFlowConstraints {
    pub fn new(n: usize) -> Self {
        let din = Dinic::new(2 + n, 0i32);
        MinFlowConstraints(din)
    }
    pub fn add_edge(&mut self, u: usize, v: usize, rng: std::ops::RangeInclusive<i32>) {
        let (l, r) = rng.into_inner();
        if l > 0 {
            self.0.add_edge(0, 2 + v, l);
            self.0.add_edge(2 + u, 1, l);
        }
        if l < r {
            self.0.add_edge(2 + u, 2 + v, r - l);
        }
    }
    pub fn is_feasible(&mut self, s: usize, t: usize) -> bool {
        let inf = (1 << 30) - 1;
        let din = &mut self.0;
        din.add_edge(2 + t, 2 + s, inf);
        let _ = din.max_flow(0, 1);
        // Check if edges of type 0 -> ? and ? -> 1 are maximally populated
        for e in &din.graph[0] {
            if e.cap != 0 {
                return false;
            }
        }
        for i in 2..din.graph.len() {
            for e in &din.graph[i] {
                if e.to == 1 && e.cap != 0 {
                    return false;
                }
            }
        }
        true
    }
}
