// Submodular minimization (up to 2-variable constraints)
// Ref: https://theory-and-me.hatenablog.com/entry/2020/03/17/180157
// Verified by: https://atcoder.jp/contests/abc259/submissions/33771580
// Depends on: graph/Dinic.rs
struct SubmodMin(Dinic<i64>, i64);

impl SubmodMin {
    fn new(n: usize) -> Self {
        let din = Dinic::new(2 + n, 0);
        SubmodMin(din, 0)
    }
    fn add1(&mut self, i: usize, cost: [i64; 2]) {
        let d = cost[1] - cost[0];
        if cost[0] < cost[1] {
            self.0.add_edge(0, 2 + i, d);
            self.1 += cost[0];
        }
        if cost[0] > cost[1] {
            self.0.add_edge(2 + i, 1, -d);
            self.1 += cost[1];
        }
        if cost[0] == cost[1] {
            self.1 += cost[1];
        }
    }
    fn add2(&mut self, i: usize, j: usize, c: [[i64; 2]; 2]) {
        assert!(c[0][0] + c[1][1] <= c[0][1] + c[1][0]);
        self.1 += c[0][0];
        self.add1(i, [0, c[1][0] - c[0][0]]);
        self.add1(j, [0, c[1][1] - c[1][0]]);
        self.0.add_edge(2 + i, 2 + j, c[0][1] + c[1][0] - (c[0][0] + c[1][1]));
    }
    #[allow(unused)]
    fn calc(&mut self) -> i64 {
        let ans = self.0.max_flow(0, 1).0;
        ans + self.1
    }
    #[allow(unused)]
    fn calc_with_sol(&mut self) -> (i64, Vec<bool>) {
        let (ans, cut) = self.0.max_flow(0, 1);
        let mut sol = vec![false; self.0.graph.len() - 2];
        for v in cut.t() {
            if v >= 2 {
                sol[v - 2] = true;
            }
        }
        (ans + self.1, sol)
    }
}
