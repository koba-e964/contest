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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
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
        w: usize, h: usize,
        s: [[i32; w]; h],
    }
    let mut uf = UnionFind::new((h + 2) * (w + 2));
    let mut b = vec![false; (h + 2) * (w + 2)];
    for i in 0..h {
        for j in 0..w {
            b[(i + 1) * (w + 2) + j + 1] = s[i][j] == 1;
        }
    }
    let mut g = vec![vec![]; (h + 2) * (w + 2)];
    for i in 0..h + 2 {
        for j in 0..w + 1 {
            let v = i * (w + 2) + j;
            g[v].push(v + 1);
            g[v + 1].push(v);
        }
    }
    for i in 0..h + 1 {
        for j in 0..w + 2 {
            let v = i * (w + 2) + j;
            g[v].push(v + w + 2);
            g[v + w + 2].push(v);
        }
        for j in 0..w + 1 {
            let v = i * (w + 2) + j;
            if i % 2 == 0 {
                g[v + 1].push(v + w + 2);
                g[v + w + 2].push(v + 1);
            } else {
                g[v].push(v + w + 3);
                g[v + w + 3].push(v);
            }
        }
    }
    for i in 0..(h + 2) * (w + 2) {
        for &w in &g[i] {
            if !b[i] && !b[w] {
                uf.unite(i, w);
            }
        }
    }
    let mut ans = 0;
    for i in 0..(h + 2) * (w + 2) {
        if !uf.is_same_set(0, i) { continue; }
        for &w in &g[i] {
            if !uf.is_same_set(i, w) { ans += 1 }
        }
    }
    println!("{}", ans);
}
