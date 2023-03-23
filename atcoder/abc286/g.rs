// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes.by_ref().map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
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
    input! {
        n: usize, m: usize,
        uv: [(usize1, usize1); m],
        k: usize,
        x: [usize1; k],
    }
    let mut pop = vec![false; m];
    for &x in &x {
        pop[x] = true;
    }
    let mut uf = UnionFind::new(n);
    for i in 0..m {
        if !pop[i] {
            let (u, v) = uv[i];
            uf.unite(u, v);
        }
    }
    let mut d = vec![0; n];
    let mut new = UnionFind::new(n);
    for &x in &x {
        let (u, v) = uv[x];
        let u = uf.root(u);
        let v = uf.root(v);
        new.unite(u, v);
        d[u] += 1;
        d[v] += 1;
    }
    let mut rs = vec![];
    for &x in &x {
        let (u, _) = uv[x];
        let u = uf.root(u);
        rs.push(new.root(u));
    }
    rs.dedup();
    if rs.len() <= 1 && d.iter().filter(|&&d| d % 2 != 0).count() <= 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
