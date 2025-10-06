// Union-Find tree.
// Verified by https://atcoder.jp/contests/pakencamp-2019-day3/submissions/9253305
struct UnionFind { disj: Vec<usize>, rank: Vec<usize> }

impl UnionFind {
    fn new(n: usize) -> Self {
        let disj = (0..n).collect();
        UnionFind { disj: disj, rank: vec![1; n] }
    }
    fn root(&mut self, x: usize) -> usize {
        if x != self.disj[x] {
            let par = self.disj[x];
            let r = self.root(par);
            self.disj[x] = r;
        }
        self.disj[x]
    }
    fn unite(&mut self, x: usize, y: usize) {
        let mut x = self.root(x);
        let mut y = self.root(y);
        if x == y { return }
        if self.rank[x] > self.rank[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.disj[x] = y;
        self.rank[y] += self.rank[x];
    }
    #[allow(unused)]
    fn is_same_set(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    #[allow(unused)]
    fn size(&mut self, x: usize) -> usize {
        let x = self.root(x);
        self.rank[x]
    }
}


// Dijkstra's algorithm.
// Verified by: AtCoder ABC164 (https://atcoder.jp/contests/abc164/submissions/12423853)
struct Dijkstra {
    edges: Vec<Vec<(usize, i64)>>, // adjacent list representation
}

impl Dijkstra {
    fn new(n: usize) -> Self {
        Dijkstra { edges: vec![Vec::new(); n] }
    }
    fn add_edge(&mut self, from: usize, to: usize, cost: i64) {
        self.edges[from].push((to, cost));
    }
    // This function returns a Vec consisting of the distances from vertex source.
    fn solve(&self, source: usize, inf: i64) -> Vec<i64> {
        let n = self.edges.len();
        let mut d = vec![inf; n];
        // que holds (-distance, vertex), so that que.pop() returns the nearest element.
        let mut que = std::collections::BinaryHeap::new();
        que.push((0, source));
        while let Some((cost, pos)) = que.pop() {
            let cost = -cost;
            if d[pos] <= cost {
                continue;
            }
            d[pos] = cost;
            for &(w, c) in &self.edges[pos] {
                let newcost = cost + c;
                if d[w] > newcost {
                    d[w] = newcost + 1;
                    que.push((-newcost, w));
                }
            }
        }
        return d;
    }
}

fn valid(g: &[u32]) -> bool {
    let mut sums = vec![];
    let n = g.len();
    let mut dijk = Dijkstra::new(n);
    let mut uf = UnionFind::new(n);
    for i in 0..n {
        for j in 0..n {
            if (g[i] & 1 << j) != 0 {
                dijk.add_edge(i, j, 1);
                uf.unite(i, j);
            }
        }
    }
    let mut r = 0;
    for i in 0..n {
        if uf.root(i) == i {
            r += 1;
        }
    }
    if r != 1 {
        return false;
    }
    for i in 0..n {
        let d = dijk.solve(i, 5);
        let mut s = 0;
        for j in 0..n {
            if i != j && d[j] <= 2 {
                s += j + 1;
            }
        }
        sums.push(s);
    }
    sums.dedup();
    sums.len() == 1
}

fn main() {
    for n in 3..=9 {
        let mut ok = false;
        for bits in 0..1i64 << (n * (n - 1) / 2) {
            let mut g = vec![0; n];
            let mut pos = 0;
            for i in 0..n {
                for j in i + 1..n {
                    if (bits & 1 << pos) != 0 {
                        g[i] |= 1 << j;
                        g[j] |= 1 << i;
                    }
                    pos += 1;
                }
            }
            if valid(&g) {
                ok = true;
                if true {
                    println!("n = {n}");
                    for i in 0..n {
                        for j in i + 1 ..n {
                            if (g[i] & 1 << j) != 0 {
                                println!("{} {}", i + 1, j + 1);
                            }
                        }
                    }
                }
                break;
            }
        }
        println!("n = {n}, ok = {ok}");
    }
}
