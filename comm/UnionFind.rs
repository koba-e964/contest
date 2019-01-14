/**
 * Union-Find tree.
 * Verified by https://atcoder.jp/contests/keyence2019/submissions/4011936
 */
struct UnionFind { disj: Vec<usize>, rank: Vec<usize> }

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut disj = vec![0; n];
        for i in 0 .. n {
            disj[i] = i;
        }
        UnionFind { disj: disj, rank: vec![0; n] }
    }
    fn root(self: &mut Self, x: usize) -> usize {
        if x != self.disj[x] {
            let par = self.disj[x];
            let r = self.root(par);
            self.disj[x] = r;
        }
        return self.disj[x];
    }
    fn unite(self: &mut Self, x: usize, y: usize) {
        let mut xr = self.root(x);
        let mut yr = self.root(y);
        if self.rank[xr] > self.rank[yr] {
            std::mem::swap(&mut xr, &mut yr);
        }
        self.disj[xr] = yr;
        self.rank[yr] = std::cmp::max(self.rank[yr], self.rank[xr] + 1);
    }
    fn is_same_set(self: &mut Self, x: usize, y: usize) -> bool {
        return self.root(x) == self.root(y);
    }
}
