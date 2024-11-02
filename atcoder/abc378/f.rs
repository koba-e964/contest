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
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn solve() {
    input! {
        n: usize,
        uv: [(usize1, usize1); n - 1],
    }
    let mut g = vec![vec![]; n];
    for &(u, v) in &uv {
        g[u].push(v);
        g[v].push(u);
    }
    let mut ex2 = vec![];
    let mut exindex = vec![n * 2; n];
    for i in 0..n {
        if g[i].len() == 2 {
            exindex[i] = ex2.len();
            ex2.push(i);
        }
    }
    let ext_idx = |v: usize, to: usize| -> usize {
        assert_eq!(g[v].len(), 2);
        let u = g[v][0];
        if u == to {
            return v;
        }
        assert_eq!(g[v][1], to);
        n + exindex[v]
    };
    let mut uf = UnionFind::new(n + ex2.len());
    for i in 0..n {
        if g[i].len() != 2 && g[i].len() != 3 {
            continue;
        }
        if g[i].len() == 3 {
            for &w in &g[i] {
                if g[w].len() == 3 {
                    uf.unite(i, w);
                }
            }
            continue;
        }
        assert_eq!(g[i].len(), 2);
        let u = g[i][0];
        let v = g[i][1];
        let mut con = |x: usize, y: usize| {
            assert_eq!(g[x].len(), 2);
            let xidx = ext_idx(x, y);
            if g[y].len() == 3 {
                uf.unite(xidx, y);
                return;
            }
            if g[y].len() != 2 {
                return;
            }
            return;
        };
        con(i, u);
        con(i, v);
    }
    let mut twos = vec![0i64; n + ex2.len()];
    for i in 0..n + ex2.len() {
        let ok = if i < n { g[i].len() == 2 } else { true };
        if ok {
            twos[uf.root(i)] += 1;
        }
    }
    let mut ans = 0i64;
    for v in twos {
        ans += v * (v - 1) / 2;
    }
    println!("{}", ans);
}
