use std::io::{Write, BufWriter};
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

// Lowest Common Ancestor. Call lca(x, y) to get the lca of them.
// Many-rooted version.
// Verified by: https://yukicoder.me/submissions/714482
// This library uses O(n) stack space. 
pub struct LCA {
    n: usize,
    bn: usize,
    parent: Vec<usize>, // r is root <=> parent[r] = r
    dep: Vec<usize>,
    lca_tbl: Vec<Vec<usize>>
}

impl LCA {
    fn dfs(&mut self, edges: &[Vec<usize>], v: usize, par: usize, d: usize) {
        self.parent[v] = par;
        self.dep[v] = d;
        for &u in edges[v].iter() {
            if u != par {
                self.dfs(edges, u, v, d + 1);
            }
        }
    }
    fn lca_init(&mut self) {
        let n = self.n;
        for v in 0 .. n {
            self.lca_tbl[v] = vec![0; self.bn + 1];
            self.lca_tbl[v][0] = self.parent[v];
        }
        for i in 1..self.bn + 1 {
            for v in 0..n {
                self.lca_tbl[v][i] =
                    self.lca_tbl[self.lca_tbl[v][i - 1]][i - 1];
            }
        }
    }
    pub fn lca(&self, mut x: usize, mut y: usize) -> usize {
        let dx = self.dep[x];
        let mut dy = self.dep[y];
        if dx > dy {
            return self.lca(y, x);
        }
        for l in (0..self.bn + 1).rev() {
            if dy - dx >= 1 << l {
                y = self.lca_tbl[y][l];
                dy -= 1 << l;
            }
        }
        assert_eq!(dx, dy);
        
        if x == y {
            return x;
        }
        for l in (0..self.bn + 1).rev() {
            if self.lca_tbl[x][l] != self.lca_tbl[y][l] {
	        x = self.lca_tbl[x][l];
	        y = self.lca_tbl[y][l];
            }
        }
        self.lca_tbl[x][0]
    }
    #[allow(unused)]
    pub fn depth(&self, a: usize) -> usize {
        self.dep[a]
    }
    #[allow(unused)]
    pub fn parent(&self, a: usize) -> usize {
        self.parent[a]
    }
    #[allow(unused)]
    pub fn ancestor(&self, v: usize, dist: usize) -> usize {
        let mut ans = v;
        for i in 0..self.bn + 1 {
            if (dist & 1 << i) != 0 {
                ans = self.lca_tbl[ans][i];
            }
        }
        ans
    }
    #[allow(unused)]
    pub fn child(&self, anc: usize, des: usize) -> usize {
        let x = self.dep[des] - self.dep[anc] - 1;
        let mut ans = des;
        for i in 0..self.bn + 1 {
            if (x & 1 << i) != 0 {
                ans = self.lca_tbl[ans][i];
            }
        }
        ans
    }
    pub fn new(edges: &[Vec<usize>], roots: &[usize]) -> Self {
        let n = edges.len();
        let bn = (n.next_power_of_two() - 1).count_ones() as usize;
        let mut ret = LCA {
            n: n, bn: bn, parent: vec![0; n], dep: vec![0; n],
            lca_tbl: vec![Vec::new(); n] };
        for &r in roots {
            ret.dfs(edges, r, r, 0);
        }
        ret.lca_init();
        ret
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        ab: [(usize1, usize1); n - 1],
        q: usize,
        uk: [(usize1, usize); q],
    }
    let mut diam = Diameter::new(n);
    for &(a, b) in &ab {
        diam.add_edge(a, b, 1);
    }
    let (x, y) = diam.diameter();
    let mut solx = vec![0; n];
    diam.dfs(x, &mut solx, None, 0);
    let mut path = vec![];
    let mut cur = y;
    while cur != x {
        let mut nxt = 0;
        for &(w, _) in &diam.edges[cur] {
            if solx[w] == solx[cur] - 1 {
                nxt = w;
                break;
            }
        }
        path.push(cur);
        cur = nxt;
    }
    path.push(x);
    eprintln!("{:?}", path);
    let mut idx = vec![n; n];
    let mut roots = vec![];
    for i in 0..path.len() {
        idx[path[i]] = i;
        roots.push(path[i]);
    }
    let mut forest = vec![vec![]; n];
    for &(u, v) in &ab {
        if idx[u] == n || idx[v] == n {
            forest[u].push(v);
            forest[v].push(u);
        }
    }
    let lca = LCA::new(&forest, &roots);
    eprintln!("dep = {:?}", lca.dep);
    let far = diam.farthest();
    for (u, k) in uk {
        if k as i64 > far[u] {
            puts!("-1\n");
            continue;
        }
        if lca.dep[u] >= k {
            // in the forest
            puts!("{}\n", lca.ancestor(u, k) + 1);
            continue;
        }
        let r = lca.ancestor(u, lca.dep[u]);
        let idx = idx[r];
        let rem = k - lca.dep[u];
        let v = if idx + rem < path.len() {
            path[idx + rem]
        } else {
            path[idx - rem]
        };
        puts!("{}\n", v + 1);
    }
}
