/**
 * Manages a tree and calculates the diameter of it.
 * Verified by: NJPC 2017-E
 *              (http://njpc2017.contest.atcoder.jp/submissions/1089492)
 */
struct Diameter {
    n: usize,
    edges: Vec<Vec<(usize, i64)>>,
    x: usize,
    y: usize,
}

impl Diameter {
    fn dfs(&self, v: usize, dist: &mut [i64], p: Option<usize>, d: i64) {
        dist[v] = d;
        for &(w, c) in self.edges[v].iter() {
            if Some(w) == p { continue; }
            self.dfs(w, dist, Some(v), d + c);
        }
    }
    pub fn new(n: usize) -> Self {
        Diameter {
            n: n,
            edges: vec![Vec::new(); n],
            x: n,
            y: n,
        }
    }
    pub fn add_edge(&mut self, a: usize, b: usize, c: i64) {
        self.edges[a].push((b, c));
        self.edges[b].push((a, c));
    }
    pub fn diameter(&mut self) -> (usize, usize) {
        let n = self.n;
        if self.x < n {
            return (self.x, self.y);
        }
        // farthest from 0
        let mut dist = vec![-1; n];
        self.dfs(0, &mut dist, None, 0);
        let mut maxi = 0;
        for i in 1 .. n {
            if dist[maxi] < dist[i] {
                maxi = i;
            }
        }
        self.x = maxi;
        // farthest from x
        self.dfs(maxi, &mut dist, None, 0);
        let mut maxi = 0;
        for i in 0 .. n {
            if dist[maxi] < dist[i] {
                maxi = i;
            }
        }
        self.y = maxi;
        (self.x, self.y)
    }
    pub fn farthest(&mut self) -> Vec<i64> {
        let n = self.n;
        if self.x >= n {
            self.diameter();
        }
        let mut ret = vec![0; n];
        let mut tmp = vec![-1; n];
        /* For every vertex, the farthest point from it is either x or y. */
        self.dfs(self.x, &mut ret, None, 0);
        self.dfs(self.y, &mut tmp, None, 0);
        for i in 0 .. n {
            ret[i] = std::cmp::max(ret[i], tmp[i]);
        }
        ret
    }
}
