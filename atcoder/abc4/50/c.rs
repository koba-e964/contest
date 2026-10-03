fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

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

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let [h, w] = ints[..] else { panic!() };
    let mut board = vec![vec!['.'; w + 2]; h + 2];
    for i in 0..h {
        for (idx, c) in getline().trim().chars().enumerate() {
            board[i + 1][idx + 1] = c;
        }
    }

    let h = h + 2;
    let w = w + 2;
    let mut uf = UnionFind::new(h * w);
    for i in 0..h {
        for j in 0..w - 1 {
            let v = i * w + j;
            if board[i][j] == '.' && board[i][j + 1] == '.' {
                uf.unite(v, v + 1);
            }
        }
    }
    for i in 0..h - 1 {
        for j in 0..w {
            let v = i * w + j;
            if board[i][j] == '.' && board[i + 1][j] == '.' {
                uf.unite(v, v + w);
            }
        }
    }
    let mut conn = 0;
    for i in 0..h {
        for j in 0..w {
            let v = i * w + j;
            if uf.root(v) == v && board[i][j] == '.' {
                conn += 1;
            }
        }
    }
    println!("{}", conn - 1);
}
